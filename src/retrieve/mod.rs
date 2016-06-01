
pub mod config;

use crypto::digest::Digest;
use crypto::md5::Md5;
use hyper::Client;
use hyper::header::Connection;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use url::Url;
use std::marker::{Sync, Send};

pub trait ImageRetriever : Send + Sync {
    fn retrieve(&self, url: &Url) -> Result<String, String>;
}

pub struct Downloader {
    config: config::Config
}

impl Downloader {
    pub fn new(config: config::Config) -> Downloader {
        Downloader { config: config }
    }

    fn download_from(&self, img_url: &Url, img_buf: &mut Vec<u8>) {
        let client = Client::new();
        let mut response = client.get(&img_url.to_string()).header(Connection::close()).send().unwrap();
        response.read_to_end(img_buf).unwrap();
    }

    fn download_path_from(&self, img_url: &Url) -> String {
        let mut hasher = Md5::new();
        let mut md5_buf = vec![0; 16];

        hasher.input(img_url.serialize().as_bytes());
        hasher.result(&mut md5_buf);

        let name_parts = img_url.path().unwrap().last().unwrap();
        let name_parts: Vec<&str> = name_parts.split('.').collect();

        let mut path_parts: Vec<String> = md5_buf.iter().map(|b| format!("{:0x}", b)).collect();
        path_parts.push(".".to_string());
        path_parts.push(name_parts.last().unwrap().to_string());

        let base_path = match self.config.get(&"base_dir") {
            & config::ConfigVal::String(ref s) => s,
            _ => panic!("Given config value for base_path is not a string")
        };
        path_parts.insert(0, base_path.to_string());
        path_parts.join("")
    }
}


impl ImageRetriever for Downloader {
    fn retrieve(&self, img_url: &Url) -> Result<String, String> {
        let mut img_buf: Vec<u8> = Vec::new();

        match img_url.scheme.as_ref() {
            "http" | "https" => self.download_from(img_url, &mut img_buf),
            _ => panic!("unsupported scheme given {:?}", img_url.scheme)
        };

        let file_path = self.download_path_from(img_url);
        let mut file = match File::create(&file_path) {
            Err(why) => panic!("error: {} ", Error::description(&why)),
            Ok(file) => file,
        };

        match file.write_all(&mut img_buf) {
            Err(why) => panic!("{:?}", Error::description(&why)),
            Ok(_) => Ok(file_path)
        }
    }
}
