use std::fmt;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::thread;
#[cfg(feature = "watch")]
use std::sync::Arc;

use camera::Camera;
use handlebars_iron::{DirectorySource, HandlebarsEngine};
#[cfg(feature = "watch")]
use handlebars_iron::Watchable;
use iron::Listening;
use iron::error::HttpResult;
use iron::prelude::*;
use router::Router;

use Result;
use config::Config;
use helper;
use handler::Index;
use watch::Heartbeat;

/// HTTP server.
pub struct Server {
    addr: SocketAddr,
    handlebars_engine: HandlebarsEngine,
    router: Router,
    template_directory: PathBuf,
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
        let cameras = try!(config.camera
            .iter()
            .map(|config| {
                Camera::new(&config.directory).map(|mut c| {
                    if let Some(name) = config.name.as_ref() {
                        c.set_name(name);
                    }
                    c
                })
            })
            .collect());

        let mut router = Router::new();
        router.get("/", Index::new(cameras, watcher.heartbeats()));

        let mut handlebars_engine = HandlebarsEngine::new();
        handlebars_engine.add(Box::new(DirectorySource::new(&config.server.template_directory, ".hbs")));
        try!(handlebars_engine.reload());
        {
            let mut registry = handlebars_engine.registry.write().unwrap();
            registry.register_helper("degrees-celsius", Box::new(helper::degrees_celsius));
            registry.register_helper("millibars", Box::new(helper::millibars));
            registry.register_helper("percentage", Box::new(helper::percentage));
            registry.register_helper("orion-percentage", Box::new(helper::orion_percentage));
        }

        Ok(Server {
            addr: addr,
            handlebars_engine: handlebars_engine,
            template_directory: PathBuf::from(&config.server.template_directory),
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

        let mut chain = Chain::new(self.router);
        chain.link_after(maybe_watch_handlebars_engine(self.template_directory,
                                                       self.handlebars_engine));
        Iron::new(chain).http(self.addr)
    }
}

#[cfg(feature = "watch")]
fn maybe_watch_handlebars_engine<P>(path: P,
                                    handlebars_engine: HandlebarsEngine)
                                    -> Arc<HandlebarsEngine>
    where P: AsRef<Path>
{
    let handlebars_engine = Arc::new(handlebars_engine);
    handlebars_engine.watch(&path.as_ref().to_string_lossy());
    handlebars_engine
}

#[cfg(not(feature = "watch"))]
fn maybe_watch_handlebars_engine<P>(_: P, handlebars_engine: HandlebarsEngine) -> HandlebarsEngine
    where P: AsRef<Path>
{
    handlebars_engine
}

impl fmt::Debug for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server {{ addr: {} }}", self.addr)
    }
}
