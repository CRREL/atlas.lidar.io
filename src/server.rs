use std::fmt;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, RwLock};

use heartbeat::Source;
use iron::Listening;
use iron::error::HttpResult;
use iron::prelude::*;
use router::Router;

use Result;
use config::Config;
use handler::Index;

/// HTTP server.
pub struct Server {
    addr: SocketAddr,
    router: Router,
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
        Server::new(&config)
    }

    fn new(config: &Config) -> Result<Server> {
        let ip = try!(config.server.ip.parse());
        let addr = SocketAddr::new(ip, config.server.port);

        let mut source = try!(Source::from_path(&config.heartbeat.directory));
        let heartbeats = Arc::new(RwLock::new(try!(source.heartbeats())));

        let mut router = Router::new();
        router.get("/", Index::new(heartbeats));

        Ok(Server {
            addr: addr,
            router: router,
        })
    }

    /// Starts the server.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use atlas_lidar_io::Server;
    /// let server = Server::from_path("data/server.toml").unwrap();
    /// server.serve().unwrap();
    /// ```
    pub fn serve(self) -> HttpResult<Listening> {
        let chain = Chain::new(self.router);
        Iron::new(chain).http(self.addr)
    }
}

impl fmt::Debug for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server {{ addr: {} }}", self.addr)
    }
}
