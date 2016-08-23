use std::fmt;
use std::net::SocketAddr;
use std::path::Path;
use std::thread;

use iron::Listening;
use iron::error::HttpResult;
use iron::prelude::*;
use router::Router;

use Result;
use config::Config;
use handler::Index;
use watch::Heartbeat;

/// HTTP server.
pub struct Server {
    addr: SocketAddr,
    router: Router,
    watcher: Heartbeat,
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

        let watcher = try!(Heartbeat::new(&config.heartbeat.directory));

        let mut router = Router::new();
        router.get("/", Index::new(watcher.heartbeats()));

        Ok(Server {
            addr: addr,
            router: router,
            watcher: watcher,
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
        let mut watcher = self.watcher;
        thread::spawn(move || watcher.watch().unwrap());
        let chain = Chain::new(self.router);
        Iron::new(chain).http(self.addr)
    }
}

impl fmt::Debug for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server {{ addr: {} }}", self.addr)
    }
}
