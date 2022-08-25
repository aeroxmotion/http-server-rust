use std::{
  net::TcpStream,
  io::{BufReader, BufRead},
  collections::HashMap,
};

pub struct Request<'a> {
  pub method: String,
  pub uri: String,
  pub version: String,
  pub headers: HashMap<String, String>,

  /// Access to the underlaying TCP stream
  pub tcp_stream: &'a TcpStream,
}

impl<'a> Request<'a> {
  pub fn parse(mut stream: &TcpStream) -> Result<Request, String> {
    let buf_reader = BufReader::new(&mut stream);
    let mut request_lines = buf_reader
      .lines()
      .map(|result| result.unwrap())
      .take_while(|line| !line.is_empty());

    let first_line = request_lines
      .next()
      .unwrap();

    let mut first_line_parts = first_line.split(" ");
    let mut next_first_line_part = || first_line_parts.next().unwrap().to_owned();

    let method = next_first_line_part();
    let uri = next_first_line_part();
    let version = next_first_line_part();

    let mut headers = HashMap::<String, String>::new();

    for request_line in request_lines {
      let mut header_parts = request_line.split(": ");

      headers.insert(
        header_parts.next().unwrap().into(),
        header_parts.next().unwrap().into(),
      );
    }

    Ok(
      Request {
        method,
        uri,
        version,
        headers,

        tcp_stream: stream,
      }
    )
  }
}
