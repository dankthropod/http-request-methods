use minreq;

fn main() -> Result<(), minreq::Error> {
   let o = minreq::get("http://httpbin.org/response-headers?foo=bar").send()?;
   let s = o.as_str()?;
   print!("{}", s);
   Ok(())
}