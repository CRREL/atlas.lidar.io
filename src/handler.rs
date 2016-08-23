use std::sync::{Arc, RwLock};

use camera::Camera;
use handlebars_iron::Template;
use heartbeat::Heartbeat;
use iron::Handler;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json::{Object, ToJson};

pub struct Index {
    cameras: Vec<Camera>,
    heartbeats: Arc<RwLock<Vec<Heartbeat>>>,
}

impl Index {
    pub fn new(cameras: Vec<Camera>, heartbeats: Arc<RwLock<Vec<Heartbeat>>>) -> Index {
        Index {
            cameras: cameras,
            heartbeats: heartbeats,
        }
    }
}

impl Handler for Index {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let heartbeats = self.heartbeats.read().unwrap();
        let heartbeat = iexpect!(heartbeats.last());
        let mut data = Object::new();
        data.insert("heartbeat".to_string(), heartbeat.to_json());
        data.insert("cameras".to_string(), self.cameras.to_json());
        let mut response = Response::new();
        response.set_mut(Template::new("index", data)).set_mut(status::Ok);
        Ok(response)
    }
}
