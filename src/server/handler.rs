
use downloader::Downloader;
use iron::prelude::*;
use iron::{Handler};
use iron::status;
use url::Url;

pub struct Serve<'a> {
    downloader: &'a Downloader<'a>
}

impl<'a> Serve<'a> {
    pub fn new(downloader: &'a Downloader) -> Serve<'a> {
        Serve { downloader: downloader }
    }
}

impl<'a> Handler for Serve<'a> {
    fn handle(&self, req: &'a mut Request) -> IronResult<Response> {
        let img_url = Url::parse("https://pbs.twimg.com/profile_images/562466745340817408/_nIu8KHX.jpeg").unwrap();
        println!("{:?}", req.url.query);
        match self.downloader.download(&img_url) {
            Ok(f) => Ok(Response::with((status::Ok, format!("{:?}", f)))),
            Err(_) => Ok(Response::with(status::NotFound))
        }
    }
}
