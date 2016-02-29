
use retrieve::ImageRetriever;
use iron::prelude::*;
use iron::Handler;
use iron::status;
use url::Url;
use std::any::Any;

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
        let img_url = Url::parse("https://pbs.twimg.com/profile_images/562466745340817408/_nIu8KHX.jpeg").unwrap();
        println!("{:?}", req.url.query);
        match self.img_retriever.retrieve(&img_url) {
            Ok(f) => Ok(Response::with((status::Ok, format!("{:?}", f)))),
            Err(_) => Ok(Response::with(status::NotFound))
        }
    }
}
