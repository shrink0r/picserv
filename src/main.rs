extern crate hyper;
extern crate url;
extern crate crypto;
extern crate iron;
extern crate router;
extern crate urlencoded;

mod retrieve;
mod serve;

use retrieve::Downloader;
use retrieve::config::Config;
use serve::Serve;
use iron::Chain;
use iron::Iron;
use router::Router;

fn main() {
    let mut router = Router::new();
    router.get("/", Serve::new(Downloader::new(Config::new())));

    let chain = Chain::new(router);
    Iron::new(chain).http("localhost:3000").unwrap();
}
