use handlebars_iron::Template;
use iron::Handler;
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
