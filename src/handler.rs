use handlebars_iron::Template;
use iron::Handler;
use iron::mime::Mime;
use iron::prelude::*;
use iron::status;
use rustc_serialize::json::ToJson;

use world::World;

pub struct Index {
    world: World,
}

impl Index {
    pub fn new(world: World) -> Index {
        Index { world: world }
    }
}

impl Handler for Index {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut response = Response::new();
        response.set_mut(Template::new("index", self.world.to_json())).set_mut(status::Ok);
        Ok(response)
    }
}

pub struct Gif {
    world: World,
}

impl Gif {
    pub fn new(world: World) -> Gif {
        Gif { world: world }
    }
}

impl Handler for Gif {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let gif = match utils::camera_name(request.url.path())
            .and_then(|name| self.world.gifs.get(name)) {
            Some(g) => g,
            None => return Ok(Response::with((status::NotFound, "no gif by that name"))),
        };
        let gif = gif.read().unwrap();
        if gif.is_empty() {
            return Ok(Response::with((status::ServiceUnavailable, "gif is empty")));
        }
        let mut response = Response::new();
        response.set_mut("image/gif".parse::<Mime>().unwrap())
            .set_mut(status::Ok)
            .set_mut(gif.clone());
        Ok(response)
    }
}

mod utils {
    use std::path::Path;

    pub fn camera_name(mut path: Vec<&str>) -> Option<&str> {
        if path.len() > 1 {
            return None;
        }
        path.pop().map(|s| Path::new(s)).and_then(|p| {
            p.extension().and_then(|e| if e == "gif" {
                Some(p.file_stem().unwrap().to_str().unwrap())
            } else {
                None
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::utils::camera_name;

    #[test]
    fn camera_name_no_path() {
        assert!(camera_name(vec![]).is_none());
    }

    #[test]
    fn camera_name_good_path() {
        assert_eq!("ATLAS_CAM", camera_name(vec!["ATLAS_CAM.gif"]).unwrap());
    }

    #[test]
    fn camera_name_bad_extension() {
        assert!(camera_name(vec!["ATLAS_CAM.jpg"]).is_none());
    }

    #[test]
    fn camera_name_subdirectory() {
        assert!(camera_name(vec!["subdir", "ATLAS_CAM.gif"]).is_none());
    }
}
