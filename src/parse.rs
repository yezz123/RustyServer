use std::io::Read;
use std::net::TcpStream;

use crate::types::{HttpParseError, HttpVersion, Method, Request};

pub fn parse(stream: &mut TcpStream) -> Result<Request, HttpParseError> {
    let mut buf = [0u8; 4096];
    // TODO: handle partial reads

    match stream.read(&mut buf) {
        Err(err) => Err(HttpParseError::Other(format!("{}", err))),
        Ok(_) => Ok(internal_parse(String::from_utf8_lossy(&buf).into_owned())?),
    }
}

pub fn internal_parse(req: String) -> Result<Request, HttpParseError> {
    // TODO: split properly on \r\n to read headers
    let mut strings = req.split("\r\n").next().unwrap().split(' ');

    let method = get_method(strings.next())?;
    let path = get_path(strings.next())?;
    let http_version = get_http_version(strings.next())?;

    // validate_crlf; adjust tests too

    Ok(Request {
        method,
        path,
        http_version,
    })
}

fn get_http_version(req: Option<&str>) -> Result<HttpVersion, HttpParseError> {
    if let Some(version) = req {
        match version {
            "HTTP/1.1" => Ok(HttpVersion::Http1_1),
            "HTTP/2.0" => Ok(HttpVersion::Http2_0),
            _ => Err(HttpParseError::InvalidHttpVersion),
        }
    } else {
        Err(HttpParseError::InvalidHttpVersion)
    }
}

pub fn get_path(req: Option<&str>) -> Result<String, HttpParseError> {
    let string = req.ok_or(HttpParseError::InvalidPath)?.to_owned();

    if string.is_empty() {
        Err(HttpParseError::InvalidPath)
    } else {
        Ok(string)
    }
}

pub fn get_method(method: Option<&str>) -> Result<Method, HttpParseError> {
    if let Some(method) = method {
        Ok(match method {
            "GET" => Method::Get,
            "DELETE" => Method::Delete,
            "HEAD" => Method::Head,
            "OPTIONS" => Method::Options,
            "PATCH" => Method::Patch,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            _ => return Err(HttpParseError::InvalidMethod),
        })
    } else {
        Err(HttpParseError::InvalidMethod)
    }
}
