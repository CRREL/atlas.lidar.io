use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;
use std::path::{Path, PathBuf};

use camera;
use chrono::{Timelike, UTC};
use heartbeat;
use notify::{Event, RecommendedWatcher, Watcher};

use Result;

fn watch<P, F>(path: P, mut update: F) -> Result<()>
    where P: AsRef<Path>,
          F: FnMut() -> Result<()>
{
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(tx));
    try!(watcher.watch(&path));
    loop {
        match rx.recv() {
            Ok(Event { path: Some(path), op: Ok(_) }) => {
                if let Ok(metadata) = path.metadata() {
                    if metadata.is_dir() {
                        try!(watcher.unwatch(&path));
                        try!(watcher.watch(&path));
                    }
                    try!(update());
                }
            }
            // TODO report
            Err(_) => unimplemented!(),
            _ => (),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Heartbeat {
    directory: PathBuf,
    source: heartbeat::FilesystemSource,
    heartbeats: Arc<RwLock<Vec<heartbeat::Heartbeat>>>,
}

impl Heartbeat {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Heartbeat> {
        Ok(Heartbeat {
            directory: path.as_ref().to_path_buf(),
            heartbeats: Arc::new(RwLock::new(Vec::new())),
            source: try!(heartbeat::Source::from_path(&path)),
        })
    }

    pub fn heartbeats(&self) -> Arc<RwLock<Vec<heartbeat::Heartbeat>>> {
        self.heartbeats.clone()
    }

    pub fn watch(&mut self) -> Result<()> {
        try!(self.update());
        watch(self.directory.clone(), || self.update())
    }

    fn update(&mut self) -> Result<()> {
        let new_heartbeats = try!(self.source.heartbeats());
        let mut heartbeats = self.heartbeats.write().unwrap();
        heartbeats.clear();
        heartbeats.extend(new_heartbeats.into_iter());
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Gif {
    camera: camera::Camera,
    gif_maker: camera::Gif,
    hours: Vec<u32>,
    days: i64,
    gif: Arc<RwLock<Vec<u8>>>,
}

impl Gif {
    pub fn new(gif: camera::Gif, camera: camera::Camera, hours: Vec<u32>, days: i64) -> Gif {
        Gif {
            gif_maker: gif,
            camera: camera,
            hours: hours,
            days: days,
            gif: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn gif(&self) -> Arc<RwLock<Vec<u8>>> {
        self.gif.clone()
    }

    pub fn watch(&mut self) -> Result<()> {
        try!(self.update());
        let directory = self.camera.directory().to_path_buf();
        watch(directory, || self.update())
    }

    pub fn update(&mut self) -> Result<()> {
        let now = UTC::now();
        let files: Vec<PathBuf> = try!(self.camera.images())
            .into_iter()
            .filter_map(|i| if (now - i.datetime).num_days() < self.days &&
                               self.hours.contains(&i.datetime.hour()) {
                Some(i.path)
            } else {
                None
            })
            .collect();
        let new_gif = try!(self.gif_maker.create(files));
        let mut gif = self.gif.write().unwrap();
        gif.clear();
        gif.extend(new_gif.into_iter());
        Ok(())
    }
}
