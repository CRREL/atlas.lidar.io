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

pub fn li_status(_: &Context,
                 h: &Helper,
                 _: &Handlebars,
                 rc: &mut RenderContext)
                 -> Result<(), RenderError> {
    let status = h.param(0).unwrap().value().as_string().unwrap();
    let name = h.param(1).unwrap().value().as_string().unwrap();
    let glyphicon = match status {
        "ok" => "ok",
        "late" => "time",
        "error" => "remove",
        "missing" => "flag",
        _ => panic!("Invalid status text: {}", status),
    };
    try!(write!(rc.writer,
                "<li class=\"status-report-{status}\"><span class=\"glyphicon glyphicon-{glyphicon}\" aria-hiddien=\"true\"></span><span class=\"status-report-name\">{name}</span><span class=\"status-report-status\">{status}</span></li>",
                name = name,
                status = status,
                glyphicon = glyphicon));
    Ok(())
}
