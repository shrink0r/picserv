
use retrieve::ImageRetriever;
use iron::prelude::*;
use iron::Handler;
use iron::status;
use url::Url;

pub struct Serve {
    img_retriever: ImageRetriever
}

impl Serve {
    pub fn new(img_retriever: ImageRetriever) -> Serve {
        Serve { img_retriever: img_retriever }
    }
}

impl Handler for Serve {
    fn handle(&self, req: & mut Request) -> IronResult<Response> {
        let img_url = Url::parse("https://pbs.twimg.com/profile_images/562466745340817408/_nIu8KHX.jpeg").unwrap();
        println!("{:?}", req.url.query);
        match self.img_retriever.retrieve(&img_url) {
            Ok(f) => Ok(Response::with((status::Ok, format!("{:?}", f)))),
            Err(_) => Ok(Response::with(status::NotFound))
        }
    }
}
