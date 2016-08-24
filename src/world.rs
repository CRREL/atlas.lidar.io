use std::sync::{Arc, RwLock};
use std::thread;

use heartbeat::Heartbeat;
use rustc_serialize::json::{Json, Object, ToJson};

use {Result, config, watch};

#[derive(Clone, Debug)]
pub struct World {
    heartbeats: Arc<RwLock<Vec<Heartbeat>>>,
    heartbeat_watcher: watch::Heartbeat,
}

impl World {
    pub fn new(heartbeat: config::Heartbeat, _: Vec<config::Camera>) -> Result<World> {
        let heartbeat_watcher = try!(watch::Heartbeat::new(heartbeat.directory));
        Ok(World {
            heartbeats: heartbeat_watcher.heartbeats(),
            heartbeat_watcher: heartbeat_watcher,
        })
    }

    pub fn serve(mut self) {
        thread::spawn(move || self.heartbeat_watcher.watch().unwrap());
    }
}

impl ToJson for World {
    fn to_json(&self) -> Json {
        let mut world = Object::new();
        if let Some(heartbeat) = self.heartbeats.read().unwrap().last() {
            world.insert("heartbeat".to_string(), heartbeat.to_json());
        }
        Json::Object(world)
    }
}
