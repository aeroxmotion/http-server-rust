// use regex::Regex;
use std::{
  net::TcpStream,
  collections::HashMap,
  io::{Error as IoError, Write},
};

pub enum StatusCode {
  OK,
}

pub struct Response<'a> {
  pub status: StatusCode,
  pub headers: HashMap<String, String>,

  /// Access to the underlaying TCP stream
  pub tcp_stream: &'a TcpStream,
}

impl<'a> Response<'a> {
  fn compute_response(&self, body: String) -> String {
    // let re = Regex::new(r"^([a-z]+)|(-[a-z]+)").unwrap();
    let mut response = String::new();

    response.push_str(format!("HTTP/1.1 {} {}\r\n", "200", "OK").as_str());

    for (name, value) in self.headers.clone() {
      response.push_str(format!("{name}: {value}\r\n").as_str());
    }

    response.push_str(format!("\r\n{body}").as_str());
    response
  }
}

impl<'a> Response<'a> {
  pub fn new(tcp_stream: &TcpStream) -> Response {
    Response {
      status: StatusCode::OK,
      headers: HashMap::new(),
      tcp_stream,
    }
  }

  pub fn set(&mut self, name: &str, value: &str) {
    self.headers.insert(name.into(), value.into());
  }

  pub fn get(&self, name: &str) -> Option<&String> {
    self.headers.get(name.into())
  }

  pub fn send(&mut self, body: String) -> Result<(), IoError> {
    let response = self.compute_response(body);

    self.tcp_stream.write_all(response.as_bytes())
  }
}
