use std::io;
use std::net::AddrParseError;

use heartbeat;
use toml::{DecodeError, ParserError};

/// Crate-specific error enum.
///
/// TODO derive Error.
#[derive(Debug)]
pub enum Error {
    /// Wrapper around `std::io::Error`.
    Io(io::Error),
    /// Wrapper around `heartbeat::Error`.
    Heartbeat(heartbeat::Error),
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

impl From<heartbeat::Error> for Error {
    fn from(err: heartbeat::Error) -> Error {
        Error::Heartbeat(err)
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
