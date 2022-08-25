use std::fs;
use server::Server;

mod request;
mod response;
mod server;

fn main() {
  let server = Server::new("127.0.0.1:7878");

  server.listen(|req, mut res| {
    if req.method == "GET" {
      println!("In a GET method... Headers: {:#?}", req.headers);
    }
  
    res.set("Content-Type", "text/html");
  
    res
      .send(fs::read_to_string("hello.html").unwrap())
      .unwrap();
  });
}
