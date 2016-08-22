use std::net::SocketAddr;
use std::path::Path;

use iron::Listening;
use iron::error::HttpResult;
use iron::prelude::*;

use Result;
use config::Config;
use handler::Index;

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
    /// ```no_run
    /// # use atlas_lidar_io::Server;
    /// let server = Server::from_path("data/server.toml").unwrap();
    /// server.serve().unwrap();
    /// ```
    pub fn serve(&self) -> HttpResult<Listening> {
        Iron::new(self.chain()).http(self.addr())
    }

    fn chain(&self) -> Chain {
        Chain::new(Index)
    }

    fn addr(&self) -> SocketAddr {
        self.addr
    }
}
