use std::fmt;
use std::net::SocketAddr;
use std::path::Path;
#[cfg(feature = "watch")]
use std::sync::Arc;

use handlebars_iron::{DirectorySource, HandlebarsEngine};
#[cfg(feature = "watch")]
use handlebars_iron::Watchable;
use iron::Listening;
use iron::error::HttpResult;
use iron::prelude::*;
use mount::Mount;
use staticfile::Static;

use Result;
use api::Api;
use config::Config;
use handler::{Gif, Index};
use helper;
use world::World;

/// HTTP server.
pub struct Server {
    addr: SocketAddr,
    chain: Chain,
    world: World,
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
        Server::new(config)
    }

    fn new(config: Config) -> Result<Server> {
        let world = try!(World::new(config.heartbeat, config.camera, config.gif));
        let config = config.server;

        let ip = try!(config.ip.parse());
        let addr = SocketAddr::new(ip, config.port);

        let mut mount = Mount::new();
        if let Some(static_directory) = config.static_directory {
            mount.mount("/static/", Static::new(&static_directory));
        }
        mount.mount("/api/v1/", Api::new(world.clone()));
        mount.mount("/gif/", Gif::new(world.clone()));
        mount.mount("/", Index::new(world.clone()));
        let mut chain = Chain::new(mount);

        let mut handlebars_engine = HandlebarsEngine::new();
        handlebars_engine.add(Box::new(DirectorySource::new(&config.template_directory, ".hbs")));
        try!(handlebars_engine.reload());
        {
            let mut registry = handlebars_engine.registry.write().unwrap();
            registry.register_helper("degrees-celsius", Box::new(helper::degrees_celsius));
            registry.register_helper("millibars", Box::new(helper::millibars));
            registry.register_helper("percentage", Box::new(helper::percentage));
            registry.register_helper("orion-percentage", Box::new(helper::orion_percentage));
            registry.register_helper("commas", Box::new(helper::commas));
            registry.register_helper("battery-charge",
                                     // TODO configure?
                                     Box::new(helper::BatteryCharge {
                                         green_floor: 70.,
                                         yellow_floor: 50.,
                                     }));
            registry.register_helper("efoy", Box::new(helper::efoy));
        }
        chain.link_after(maybe_watch_handlebars_engine(&config.template_directory,
                                                       handlebars_engine));

        Ok(Server {
            addr: addr,
            chain: chain,
            world: world,
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
        self.world.serve();
        Iron::new(self.chain).http(self.addr)
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
