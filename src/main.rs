extern crate atlas_lidar_io;
extern crate docopt;
extern crate rustc_serialize;
extern crate toml;

use docopt::Docopt;

use atlas_lidar_io::Server;

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
    arg_config: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .map(|d| d.version(option_env!("CARGO_PKG_VERSION").map(|s| s.to_string())))
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let server = Server::from_path(&args.arg_config).unwrap();
    server.serve().unwrap();
}
