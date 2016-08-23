use std::sync::{Arc, RwLock};

use camera::Camera;
use chrono::UTC;
use handlebars_iron::Template;
use heartbeat::Heartbeat;
use iron::Handler;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json::{Json, Object, ToJson};

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

    fn status_report(&self) -> StatusReport {
        let mut report = Object::new();
        if let Some(heartbeat) = self.heartbeats.read().unwrap().last() {
            let now = UTC::now();
            report.insert("heartbeat".to_string(),
                          {
                                  let duration = now - heartbeat.start_time;
                                  if duration.num_minutes() < 70 {
                                      Status::Ok
                                  } else if duration.num_hours() < 3 {
                                      Status::Late
                                  } else {
                                      Status::Stopped
                                  }
                              }
                              .to_json());
            report.insert("scan".to_string(),
                          {
                                  let duration = now - heartbeat.last_scan.start;
                                  if duration.num_hours() < 7 {
                                      Status::Ok
                                  } else if duration.num_hours() < 24 {
                                      Status::Late
                                  } else {
                                      Status::Stopped
                                  }
                              }
                              .to_json());
        } else {
            report.insert("heartbeat".to_string(), Status::Missing.to_json());
            report.insert("scan".to_string(), Status::Missing.to_json());
        }
        report
    }
}

type StatusReport = Object;

enum Status {
    Ok,
    Late,
    Stopped,
    Missing,
}

impl ToJson for Status {
    fn to_json(&self) -> Json {
        let name = match *self {
            Status::Ok => "ok",
            Status::Late => "late",
            Status::Stopped => "stopped",
            Status::Missing => "missing",
        };
        Json::String(name.to_string())
    }
}

impl Handler for Index {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let heartbeats = self.heartbeats.read().unwrap();
        let heartbeat = iexpect!(heartbeats.last());
        let mut data = Object::new();
        data.insert("status_report".to_string(), self.status_report().to_json());
        data.insert("heartbeat".to_string(), heartbeat.to_json());
        let mut cameras = self.cameras.to_json();
        if let Some(camera) = cameras.as_array_mut().unwrap().first_mut() {
            camera.as_object_mut().unwrap().insert("active".to_string(), "active".to_json());
        }
        data.insert("cameras".to_string(), cameras);
        let mut response = Response::new();
        response.set_mut(Template::new("index", data)).set_mut(status::Ok);
        Ok(response)
    }
}
