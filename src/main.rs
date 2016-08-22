extern crate atlas_lidar_io;
extern crate docopt;
extern crate rustc_serialize;
extern crate toml;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use docopt::Docopt;
use rustc_serialize::Decodable;
use toml::{Decoder, Parser, Value};

use atlas_lidar_io::{Config, Server};

const USAGE: &'static str = "
ATLAS HTTP server.

Usage:
    atlas-lidar-io <config>

Options:
    -h --help           Show this screen.
    --version           Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_config: PathBuf,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .map(|d| d.version(option_env!("CARGO_PKG_VERSION").map(|s| s.to_string())))
        .and_then(|d| d.decode())
        .unwrap();
    let mut toml = String::new();
    {
        let mut file = File::open(&args.arg_config)
            .expect(&format!("Unable to open configuration file: {}",
                             args.arg_config.to_string_lossy()));
        file.read_to_string(&mut toml).unwrap();
    }
    let mut parser = Parser::new(&toml);
    let toml = match parser.parse() {
        Some(table) => Value::Table(table),
        None => {
            for error in &parser.errors {
                println!("Parser error: {}", error);
            }
            std::process::exit(1);
        }
    };
    let mut decoder = Decoder::new(toml);
    let config = Config::decode(&mut decoder).unwrap();
    let server = Server::new(config);
    server.serve();
}
