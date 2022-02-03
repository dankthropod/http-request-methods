extern crate curl;

use curl::http;

fn main(){
  let resp = http::handle()
    .post("http://localhost:3000/login", "username=dude&password=sikrit")
    .exec().unwrap();

  println!("code={}; headers={}; body={}",
    resp.get_code(), resp.get_headers(), resp.get_body());    

}