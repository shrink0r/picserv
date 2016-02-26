extern crate hyper;
extern crate url;
extern crate crypto;
extern crate iron;

mod downloader;
mod server;

use downloader::Downloader;
use downloader::config::Config;
use server::Server;

fn main() {
    let config = Config::new();
    let downloader = Downloader::new(&config);
    let server = Server::new(&downloader);
    server.listen("localhost:3000".as_ref());
}
