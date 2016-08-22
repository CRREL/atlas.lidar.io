use std::fs::File;
use std::io::Read;
use std::path::Path;

use rustc_serialize::Decodable;
use toml::{Decoder, Parser, Value};

use {Error, Result};

#[derive(Debug, RustcDecodable)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut string = String::new();
        {
            let mut file = try!(File::open(path));
            try!(file.read_to_string(&mut string));
        }
        let mut parser = Parser::new(&string);
        let toml = match parser.parse() {
            Some(toml) => Value::Table(toml),
            None => return Err(Error::ParseConfig(parser.errors.clone())),
        };
        Config::decode(&mut Decoder::new(toml)).map_err(Error::from)
    }
}

#[derive(Debug, RustcDecodable)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}
