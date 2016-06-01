
use iron::Handler;
use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use retrieve::ImageRetriever;
use std::any::Any;
use url::Url;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use urlencoded::UrlEncodedQuery;

pub struct Serve<T> where T : ImageRetriever {
    img_retriever: T
}

impl<T> Serve<T> where T : ImageRetriever {
    pub fn new(img_retriever: T) -> Serve<T> {
        Serve { img_retriever: img_retriever }
    }
}

impl<T> Handler for Serve<T> where T : ImageRetriever + Any {
    fn handle(&self, req: & mut Request) -> IronResult<Response> {
        let params = match req.get_ref::<UrlEncodedQuery>() {
            Ok(hashmap) => hashmap,
            Err(ref e) => panic!("{:?}", e)
        };

        let img_url = match params.get("url") {
            Some(url) => Url::parse(url.first().unwrap()).unwrap(),
            None => panic!("Missing required url parameter")
        };

        let content_type = "image/jpeg".parse::<Mime>().unwrap();
        match self.img_retriever.retrieve(&img_url) {
            Ok(path) => {
                let mut data = Vec::new();
                let mut file = match File::open(path) {
                    Err(why) => panic!("error: {} ", Error::description(&why)),
                    Ok(file) => file,
                };
                let length = file.read_to_end(&mut data);
                return Ok(Response::with((content_type, status::Ok, data)));
            },
            Err(err) => {
                Ok(Response::with(status::NotFound))
            }
        }
    }
}
