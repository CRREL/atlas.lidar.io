extern crate iron;
extern crate rustc_serialize;
extern crate toml;

use std::io;
use std::net::AddrParseError;

use toml::{DecodeError, ParserError};

mod config;
mod handler;
mod server;

pub use server::Server;

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
