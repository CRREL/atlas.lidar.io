use std::sync::{Arc, RwLock};

use iron::Handler;
use iron::status;
use iron::prelude::*;
use heartbeat::Heartbeat;
use rustc_serialize::json::ToJson;

pub struct Index {
    heartbeats: Arc<RwLock<Vec<Heartbeat>>>,
}

impl Index {
    pub fn new(heartbeats: Arc<RwLock<Vec<Heartbeat>>>) -> Index {
        Index { heartbeats: heartbeats }
    }
}

impl Handler for Index {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let heartbeats = self.heartbeats.read().unwrap();
        Ok(Response::with((status::Ok, heartbeats.last().unwrap().to_json().to_string())))
    }
}
