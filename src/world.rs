use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

use camera::{Camera, Gif};
use chrono::{DateTime, Duration, UTC};
use heartbeat::Heartbeat;
use rustc_serialize::json::{Json, Object, ToJson};

use {Result, config, watch};

#[derive(Clone, Debug)]
pub struct World {
    display_cameras: Vec<DisplayCamera>,
    gif_watchers: Vec<watch::Gif>,
    pub gifs: HashMap<String, Arc<RwLock<Vec<u8>>>>,
    heartbeat_watcher: watch::Heartbeat,
    pub heartbeats: Arc<RwLock<Vec<Heartbeat>>>,
    intervals: HashMap<String, config::Interval>,
}

impl World {
    pub fn new(heartbeat_config: config::Heartbeat,
               camera_config: Vec<config::Camera>,
               gif_config: config::Gif)
               -> Result<World> {
        let heartbeat_watcher = try!(watch::Heartbeat::new(heartbeat_config.directory));

        let mut intervals = HashMap::new();
        intervals.insert("heartbeat".to_string(), heartbeat_config.interval);
        intervals.insert("scan".to_string(), heartbeat_config.scan_interval);

        let mut gif = Gif::new();
        gif.set_delay(Duration::milliseconds(gif_config.delay * 10));
        gif.set_width(gif_config.width);
        gif.set_height(gif_config.height);
        gif.set_loop(gif_config.loop_gif);

        let mut cameras = Vec::new();
        let mut gif_watchers = Vec::new();
        let mut gifs = HashMap::new();
        for config in camera_config {
            let mut camera = try!(Camera::new(config.directory));
            if let Some(name) = config.name {
                camera.set_name(&name);
            }
            if let Some(url_path) = config.url_path {
                camera.set_url_path(&url_path);
            }
            intervals.insert(camera.name().to_string(), config.interval);
            cameras.push(DisplayCamera {
                name: config.display_name.unwrap_or_else(|| camera.name().to_string()),
                camera: camera.clone(),
                gif_url: config.gif
                    .as_ref()
                    .map(|_| format!("/gif/{}.gif", camera.name().to_string())),
                gif_days: config.gif.as_ref().map(|c| c.days),
            });
            if let Some(gif_config) = config.gif {
                let name = camera.name().to_string();
                let watcher = watch::Gif::new(gif, camera, gif_config.hours, gif_config.days);
                gifs.insert(name, watcher.gif());
                gif_watchers.push(watcher);
            }
        }

        Ok(World {
            display_cameras: cameras,
            gifs: gifs,
            gif_watchers: gif_watchers,
            heartbeats: heartbeat_watcher.heartbeats(),
            heartbeat_watcher: heartbeat_watcher,
            intervals: intervals,
        })
    }

    pub fn serve(self) {
        let mut heartbeat_watcher = self.heartbeat_watcher;
        thread::spawn(move || heartbeat_watcher.watch().unwrap());
        for mut watcher in self.gif_watchers {
            thread::spawn(move || watcher.watch().unwrap());
        }
    }

    fn status(&self) -> Vec<Component> {
        let mut status = Vec::new();
        let heartbeats = self.heartbeats.read().unwrap();
        let heartbeat = heartbeats.last();
        status.push(Component::new("Heartbeat",
                                   Status::new(heartbeat.map(|h| h.start_time),
                                               self.intervals["heartbeat"])));
        status.push(Component::new("Scan",
                                   Status::new(heartbeat.map(|h| h.last_scan.start),
                                               self.intervals["scan"])));
        for display_camera in &self.display_cameras {
            let interval = self.intervals[display_camera.camera.name()];
            status.push(Component::new(&display_camera.name,
                                       Status::new(display_camera.camera
                                                       .latest_image()
                                                       .unwrap_or(None)
                                                       .map(|i| i.datetime),
                                                   interval)));
        }
        status
    }
}

#[derive(Clone, Debug)]
struct DisplayCamera {
    name: String,
    camera: Camera,
    gif_url: Option<String>,
    gif_days: Option<i64>,
}

impl ToJson for DisplayCamera {
    fn to_json(&self) -> Json {
        let mut camera = Object::new();
        camera.insert("name".to_string(), self.name.to_json());
        camera.insert("camera".to_string(), self.camera.to_json());
        if let Some(ref url) = self.gif_url {
            let mut gif = Object::new();
            gif.insert("url".to_string(), url.to_json());
            gif.insert("days".to_string(), self.gif_days.to_json());
            camera.insert("gif".to_string(), gif.to_json());
        }
        Json::Object(camera)
    }
}

struct Component {
    name: String,
    status: Status,
}

impl Component {
    fn new(name: &str, status: Status) -> Component {
        Component {
            name: name.to_string(),
            status: status,
        }
    }
}

impl ToJson for Component {
    fn to_json(&self) -> Json {
        let mut component = Object::new();
        component.insert("name".to_string(), self.name.to_json());
        component.insert("status".to_string(), self.status.to_json());
        Json::Object(component)
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
        world.insert("cameras".to_string(), self.display_cameras.to_json());
        Json::Object(world)
    }
}
