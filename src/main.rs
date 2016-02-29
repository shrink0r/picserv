extern crate hyper;
extern crate url;
extern crate crypto;
extern crate iron;

mod retrieve;
mod serve;

use retrieve::Downloader;
use retrieve::config::Config;
use serve::Serve;
use iron::Chain;
use iron::Iron;

fn main() {
    let chain = Chain::new(Serve::new(Downloader::new(Config::new())));
    Iron::new(chain).http("localhost:3000").unwrap();
}
