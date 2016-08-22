extern crate iron;
extern crate rustc_serialize;
extern crate toml;

use std::io::{self, Read};
use std::fs::File;
use std::net::{AddrParseError, SocketAddr};
use std::path::Path;

use iron::Listening;
use iron::error::HttpResult;
use iron::prelude::*;
use rustc_serialize::Decodable;
use toml::{DecodeError, Decoder, Parser, ParserError, Value};

#[derive(Debug)]
pub enum Error {
    TomlDecode(DecodeError),
    ParseConfig(Vec<ParserError>),
    Io(io::Error),
    NetAddrParse(AddrParseError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<AddrParseError> for Error {
    fn from(err: AddrParseError) -> Error {
        Error::NetAddrParse(err)
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Error {
        Error::TomlDecode(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, RustcDecodable)]
struct Config {
    server: ServerConfig,
}

impl Config {
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Config> {
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
struct ServerConfig {
    ip: String,
    port: u16,
}

#[derive(Debug)]
pub struct Server {
    addr: SocketAddr,
}

impl Server {
    /// Reads a server configuration file and creates the server.
    ///
    /// # Examples
    ///
    /// ```
    /// # use atlas_lidar_io::Server;
    /// let server = Server::from_path("data/server.toml").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Server> {
        let config = try!(Config::from_path(path));
        let ip = try!(config.server.ip.parse());
        Ok(Server { addr: SocketAddr::new(ip, config.server.port) })
    }

    /// Starts the server.
    ///
    /// # Examples
    ///
    /// ```
    /// # use atlas_lidar_io::Server;
    /// let server = Server::from_path("data/server.toml").unwrap();
    /// server.serve().unwrap();
    /// ```
    pub fn serve(&self) -> HttpResult<Listening> {
        Iron::new(self.chain()).http(self.addr())
    }

    fn chain(&self) -> Chain {
        unimplemented!()
    }

    fn addr(&self) -> SocketAddr {
        self.addr
    }
}
