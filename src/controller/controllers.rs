use http::{header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE}, HeaderMap, HeaderValue};
use std::{io::Write, net::TcpStream};

pub fn handle_get_stocks(mut stream: TcpStream, data: &String){
    let status_ok: &str = "HTTP/1.1 200 OK";
    let mut header = String::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    headers.insert(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, POST"));
    headers.insert(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("Content-Type"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    for (key, value) in headers.iter() {
        header += &format!("{}: {}\r\n", key, value.to_str().unwrap());
    }

    let response = format!("{status_ok}\r\n{header}\r\n\r\n{data}");

    stream.write_all(response.as_bytes()).unwrap();
}