
mod handler;

use downloader::Downloader;
use iron::prelude::*;

pub struct Server<'a> {
    downloader: &'a Downloader<'a>
}

impl<'a> Server<'a> {
    pub fn new(downloader: &'a Downloader) -> Server<'a> {
        Server { downloader: downloader }
    }

    pub fn listen(&self, dsn: &str) {
        let handler = handler::Serve::new(self.downloader);
        let chain = Chain::new(handler);
        Iron::new(chain).http(dsn).unwrap();
    }
}
