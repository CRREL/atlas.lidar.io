use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError};
use heartbeat::units::{OrionPercentage, Percentage};
use rustc_serialize::json::{Json, Object, ToJson};

pub fn degrees_celsius(_: &Context,
                       h: &Helper,
                       _: &Handlebars,
                       rc: &mut RenderContext)
                       -> Result<(), RenderError> {
    let degrees = h.param(0).unwrap().value().as_f64().unwrap();
    try!(write!(rc.writer, "{:.2} °C", degrees));
    Ok(())
}

pub fn millibars(_: &Context,
                 h: &Helper,
                 _: &Handlebars,
                 rc: &mut RenderContext)
                 -> Result<(), RenderError> {
    let millibars = h.param(0).unwrap().value().as_f64().unwrap();
    try!(write!(rc.writer, "{:.1} mbar", millibars));
    Ok(())
}

pub fn percentage(_: &Context,
                  h: &Helper,
                  _: &Handlebars,
                  rc: &mut RenderContext)
                  -> Result<(), RenderError> {
    let percentage = h.param(0).unwrap().value().as_f64().unwrap();
    try!(write!(rc.writer, "{:.2} %", percentage));
    Ok(())
}

pub fn orion_percentage(_: &Context,
                        h: &Helper,
                        _: &Handlebars,
                        rc: &mut RenderContext)
                        -> Result<(), RenderError> {
    let percentage: Percentage =
        OrionPercentage(h.param(0).unwrap().value().as_f64().unwrap() as f32).into();
    try!(write!(rc.writer, "{:.2} %", percentage.0));
    Ok(())
}

pub fn commas(_: &Context,
              h: &Helper,
              _: &Handlebars,
              rc: &mut RenderContext)
              -> Result<(), RenderError> {
    let number = h.param(0).unwrap().value().as_u64().unwrap();
    try!(write!(rc.writer, "{}", utils::commas(number)));
    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub struct BatteryCharge {
    pub green_floor: f32,
    pub yellow_floor: f32,
}

impl BatteryCharge {
    fn progress_bar_data(&self, soc: f32) -> Json {
        let mut data = Object::new();
        data.insert("color".to_string(),
                    if soc > self.green_floor {
                            "success"
                        } else if soc > self.yellow_floor {
                            "warning"
                        } else {
                            "danger"
                        }
                        .to_json());
        data.insert("soc".to_string(), soc.to_json());
        Json::Object(data)
    }
}

impl HelperDef for BatteryCharge {
    fn call(&self,
            _: &Context,
            h: &Helper,
            handlebars: &Handlebars,
            rc: &mut RenderContext)
            -> Result<(), RenderError> {
        let soc: Percentage = OrionPercentage(h.param(0).unwrap().value().as_f64().unwrap() as f32)
            .into();
        try!(write!(rc.writer,
                    "{}",
                    try!(handlebars.render("progress-bar", &self.progress_bar_data(soc.0)))));
        Ok(())
    }
}

mod utils {
    pub fn commas(n: u64) -> String {
        let mut bytes = n.to_string().into_bytes();
        bytes.reverse();
        let mut chunks: Vec<_> = bytes.chunks_mut(3)
            .map(|b| {
                b.reverse();
                String::from_utf8_lossy(b)
            })
            .collect();
        chunks.reverse();
        chunks.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;

    #[test]
    fn commas_zero() {
        assert_eq!("0", commas(0));
    }

    #[test]
    fn commas_one_thousand_and_more() {
        assert_eq!("1,000", commas(1000));
        assert_eq!("1,000,000", commas(1000000));
        assert_eq!("10,000,000", commas(10000000));
        assert_eq!("100,000,000", commas(100000000));
    }
}
