extern crate rustc_serialize;

pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, RustcDecodable)]
pub struct Config;

pub struct Server;

impl Server {
    pub fn new(config: Config) -> Server {
        unimplemented!()
    }

    pub fn serve(&self) -> Result<()> {
        unimplemented!()
    }
}
