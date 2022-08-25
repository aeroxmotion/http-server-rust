use std::net::TcpListener;

use crate::{request::Request, response::Response};

pub struct Server {
  pub tcp_listener: TcpListener,
}

impl Server {
  pub fn new(addr: &str) -> Server {
    let tcp_listener = TcpListener::bind(addr).unwrap();

    Server {
      tcp_listener,
    }
  }

  pub fn listen(&self, handler: fn(Request, Response)) {
    for stream in self.tcp_listener.incoming() {
      let stream = stream.unwrap();

      let request = Request::parse(&stream).unwrap();
      let response = Response::new(&stream);

      handler(request, response);
    }
  }
}
