use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::result;

use chrono::Duration;
use rustc_serialize::{Decodable, Decoder as RustcSerializeDecoder};
use toml::{Decoder, Parser, Value};

use {Error, Result};

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub ok: Duration,
    pub late: Duration,
}

impl Decodable for Interval {
    fn decode<D: RustcSerializeDecoder>(d: &mut D) -> result::Result<Interval, D::Error> {
        d.read_struct("Interval", 2, |d| {
            let ok = try!(d.read_struct_field("ok", 0, |d| d.read_i64()));
            let late = try!(d.read_struct_field("late", 0, |d| d.read_i64()));
            Ok(Interval {
                ok: Duration::minutes(ok),
                late: Duration::minutes(late),
            })
        })
    }
}

#[derive(Debug, RustcDecodable)]
pub struct Config {
    pub server: Server,
    pub heartbeat: Heartbeat,
    pub camera: Vec<Camera>,
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
pub struct Server {
    pub ip: String,
    pub port: u16,
    pub static_directory: String,
    pub template_directory: String,
}

#[derive(Debug, RustcDecodable)]
pub struct Heartbeat {
    pub directory: String,
    pub interval: Interval,
    pub scan_interval: Interval,
}

#[derive(Debug, RustcDecodable)]
pub struct Camera {
    pub directory: String,
    pub name: Option<String>,
    pub url_path: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_from_file() {
        let _ = Config::from_path("data/server.toml").unwrap();
    }
}
