use std::fmt::Write;

use heartbeat::Heartbeat;
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
        router.get("/soc", CsvHandler::new(world.clone(), Soc));
        router.get("/temperature", CsvHandler::new(world, Temperature));
        Api { router: router }
    }
}

impl Handler for Api {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        self.router.handle(request)
    }
}

trait Csv {
    fn header(&self) -> &str;
    fn row(&self, heartbeat: &Heartbeat) -> String;
}

struct CsvHandler<C: Csv> {
    csv: C,
    world: World,
}

impl<C: Csv> CsvHandler<C> {
    fn new(world: World, csv: C) -> CsvHandler<C> {
        CsvHandler {
            world: world,
            csv: csv,
        }
    }
}

impl<C: 'static + Csv + Send + Sync> Handler for CsvHandler<C> {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut data = String::new();
        itry!(writeln!(data, "{}", self.csv.header()));
        for heartbeat in self.world.heartbeats.read().unwrap().iter() {
            itry!(writeln!(data, "{}", self.csv.row(heartbeat)));
        }
        let mut response = Response::new();
        response.set_mut(status::Ok)
            .set_mut(data);
        response.headers
            .set(ContentType(Mime(TopLevel::Text, SubLevel::Ext("csv".to_string()), vec![])));
        Ok(response)
    }
}

struct Soc;

impl Csv for Soc {
    fn header(&self) -> &str {
        "Datetime,Battery #1,Battery #2"
    }
    fn row(&self, heartbeat: &Heartbeat) -> String {
        format!("{},{:.2},{:.2}",
                heartbeat.start_time,
                Percentage::from(heartbeat.soc1).0,
                Percentage::from(heartbeat.soc2).0)
    }
}

struct Temperature;

impl Csv for Temperature {
    fn header(&self) -> &str {
        "Datetime,External,Mount,Scanner"
    }
    fn row(&self, heartbeat: &Heartbeat) -> String {
        let mut row = format!("{},{},{:.2},",
                              heartbeat.start_time,
                              heartbeat.external_temperature
                                  .map_or("".to_string(), |t| format!("{:.2}", t.0)),
                              heartbeat.mount_temperature.0);
        if let Some(scanner_on) = heartbeat.last_scanner_on {
            row.push_str(&format!("{:2}", scanner_on.scanner_temperature.0));
        }
        row
    }
}
