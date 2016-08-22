use iron::Handler;
use iron::status;
use iron::prelude::*;

pub struct Index;

impl Handler for Index {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello world!")))
    }
}
