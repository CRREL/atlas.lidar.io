use heartbeat::units::Percentage;
use iron::Handler;
use iron::headers::ContentType;
use iron::mime::{Mime, SubLevel, TopLevel};
use iron::prelude::*;
use iron::status;
use router::Router;

use world::World;

pub struct Api {
    router: Router,
}

impl Api {
    pub fn new(world: World) -> Api {
        let mut router = Router::new();
        router.get("/soc", Soc { world: world });
        Api { router: router }
    }
}

impl Handler for Api {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        self.router.handle(request)
    }
}

struct Soc {
    world: World,
}

impl Handler for Soc {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut data = String::new();
        data.push_str("Datetime,Battery #1,Battery #2\n");
        for heartbeat in self.world.heartbeats.read().unwrap().iter() {
            data.push_str(&format!("{},{:.2},{:.2}\n",
                                   heartbeat.start_time.to_string(),
                                   Percentage::from(heartbeat.soc1).0,
                                   Percentage::from(heartbeat.soc2).0));
        }
        let mut response = Response::new();
        response.set_mut(status::Ok)
            .set_mut(data);
        response.headers
            .set(ContentType(Mime(TopLevel::Text, SubLevel::Ext("csv".to_string()), vec![])));
        Ok(response)
    }
}
