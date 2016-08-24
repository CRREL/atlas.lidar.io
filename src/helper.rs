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

pub fn commas(_: &Context,
              h: &Helper,
              _: &Handlebars,
              rc: &mut RenderContext)
              -> Result<(), RenderError> {
    let number = h.param(0).unwrap().value().as_u64().unwrap();
    try!(write!(rc.writer, "{}", utils::commas(number)));
    Ok(())
}

mod utils {
    pub fn commas(n: u64) -> String {
        let mut bytes = n.to_string().into_bytes();
        bytes.reverse();
        let mut chunks: Vec<_> = bytes.chunks(3).map(|b| String::from_utf8_lossy(b)).collect();
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
    }
}
