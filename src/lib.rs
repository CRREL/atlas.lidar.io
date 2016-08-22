//! http://atlas.lidar.io

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

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

/// Crate-specific error enum.
///
/// TODO derive Error.
#[derive(Debug)]
pub enum Error {
    /// Wrapper around `std::io::Error`.
    Io(io::Error),
    /// Wrapper around `std::net::AddrParseError`.
    NetAddrParse(AddrParseError),
    /// Wrapper around a vector of `toml::ParserError`.
    ParseConfig(Vec<ParserError>),
    /// Wrapper around `toml::DecodeError`.
    TomlDecode(DecodeError),
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

/// Crate-specific result type.
pub type Result<T> = std::result::Result<T, Error>;
