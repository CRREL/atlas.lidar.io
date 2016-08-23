use std::error;
use std::fmt;
use std::io;
use std::net::AddrParseError;

use camera;
use handlebars_iron;
use heartbeat;
use notify;
use toml::{DecodeError, ParserError};

/// Crate-specific error enum.
#[derive(Debug)]
pub enum Error {
    /// Wrapper around `camera::Error`.
    Camera(camera::Error),
    /// Wrapper around `std::io::Error`.
    Io(io::Error),
    /// Wrapper around `handlebars_iron::SourceError`.
    HandlebarsSource(handlebars_iron::SourceError),
    /// Wrapper around `heartbeat::Error`.
    Heartbeat(heartbeat::Error),
    /// Wrapper around `std::net::AddrParseError`.
    NetAddrParse(AddrParseError),
    /// Wrapper around `notify::Error`.
    Notify(notify::Error),
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

impl From<notify::Error> for Error {
    fn from(err: notify::Error) -> Error {
        Error::Notify(err)
    }
}

impl From<handlebars_iron::SourceError> for Error {
    fn from(err: handlebars_iron::SourceError) -> Error {
        Error::HandlebarsSource(err)
    }
}

impl From<camera::Error> for Error {
    fn from(err: camera::Error) -> Error {
        Error::Camera(err)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Camera(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::HandlebarsSource(ref err) => err.description(),
            Error::Heartbeat(ref err) => err.description(),
            Error::NetAddrParse(ref err) => err.description(),
            Error::Notify(ref err) => err.description(),
            Error::ParseConfig(_) => "configuration parse error",
            Error::TomlDecode(ref err) => err.description(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Camera(ref err) => write!(f, "camera error: {}", err),
            Error::Io(ref err) => write!(f, "io error: {}", err),
            Error::HandlebarsSource(ref err) => write!(f, "handlebars source error: {}", err),
            Error::Heartbeat(ref err) => write!(f, "heartbeat error: {}", err),
            Error::NetAddrParse(ref err) => write!(f, "net addr parse error: {}", err),
            Error::Notify(ref err) => write!(f, "notify error: {}", err),
            Error::ParseConfig(ref errors) => {
                write!(f,
                       "config parse error(s): {}",
                       errors.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", "))
            }
            Error::TomlDecode(ref err) => write!(f, "toml decode error: {}", err),
        }
    }
}
