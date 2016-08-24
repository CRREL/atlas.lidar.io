use handlebars::{Context, Handlebars, Helper, RenderContext, RenderError};

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
    let orion_percentage = h.param(0).unwrap().value().as_f64().unwrap();
    try!(write!(rc.writer, "{:.2} %", 100.0 * orion_percentage / 5.0));
    Ok(())
}
