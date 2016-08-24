use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, RwLock};
use std::thread;

use camera::Camera;
use chrono::{DateTime, UTC};
use heartbeat::Heartbeat;
use rustc_serialize::json::{Json, Object, ToJson};

use {Result, config, watch};

#[derive(Clone, Debug)]
pub struct World {
    cameras: Vec<Camera>,
    heartbeats: Arc<RwLock<Vec<Heartbeat>>>,
    heartbeat_watcher: watch::Heartbeat,
    intervals: HashMap<String, config::Interval>,
}

impl World {
    pub fn new(heartbeat: config::Heartbeat, cameras: Vec<config::Camera>) -> Result<World> {
        let heartbeat_watcher = try!(watch::Heartbeat::new(heartbeat.directory));
        let mut intervals = HashMap::new();
        intervals.insert("heartbeat".to_string(), heartbeat.interval);
        intervals.insert("scan".to_string(), heartbeat.scan_interval);
        let cameras = try!(cameras.into_iter()
            .map(|c| {
                let mut camera = try!(Camera::new(c.directory));
                if let Some(name) = c.name {
                    camera.set_name(&name);
                }
                if let Some(url_path) = c.url_path {
                    camera.set_url_path(&url_path);
                }
                Ok(camera)
            })
            .collect::<Result<_>>());
        Ok(World {
            cameras: cameras,
            heartbeats: heartbeat_watcher.heartbeats(),
            heartbeat_watcher: heartbeat_watcher,
            intervals: intervals,
        })
    }

    pub fn serve(mut self) {
        thread::spawn(move || self.heartbeat_watcher.watch().unwrap());
    }

    fn status(&self) -> BTreeMap<String, Status> {
        let mut status = BTreeMap::new();
        let heartbeats = self.heartbeats.read().unwrap();
        let heartbeat = heartbeats.last();
        status.insert("Heartbeat".to_string(),
                      Status::new(heartbeat.map(|h| h.start_time), self.intervals["heartbeat"]));
        status.insert("Scan".to_string(),
                      Status::new(heartbeat.map(|h| h.last_scan.start), self.intervals["scan"]));
        status
    }
}

enum Status {
    Ok,
    Late,
    Stopped,
    Missing,
}

impl Status {
    fn new(datetime: Option<DateTime<UTC>>, interval: config::Interval) -> Status {
        if let Some(datetime) = datetime {
            let duration = UTC::now() - datetime;
            if duration < interval.ok {
                Status::Ok
            } else if duration < interval.late {
                Status::Late
            } else {
                Status::Stopped
            }
        } else {
            Status::Missing
        }
    }
}

impl ToJson for Status {
    fn to_json(&self) -> Json {
        let mut status = Object::new();
        status.insert("name".to_string(),
                      match *self {
                              Status::Ok => "ok",
                              Status::Late => "late",
                              Status::Stopped => "stopped",
                              Status::Missing => "missing",
                          }
                          .to_json());
        status.insert("glyphicon".to_string(),
                      match *self {
                              Status::Ok => "ok",
                              Status::Late => "time",
                              Status::Stopped => "remove",
                              Status::Missing => "flag",
                          }
                          .to_json());
        Json::Object(status)
    }
}

impl ToJson for World {
    fn to_json(&self) -> Json {
        let mut world = Object::new();
        world.insert("status".to_string(), self.status().to_json());
        if let Some(heartbeat) = self.heartbeats.read().unwrap().last() {
            world.insert("heartbeat".to_string(), heartbeat.to_json());
        }
        world.insert("cameras".to_string(), self.cameras.to_json());
        Json::Object(world)
    }
}
