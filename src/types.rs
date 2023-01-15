use std::net::TcpListener;

pub struct Server {
    pub(crate) listener: TcpListener,
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Head,
    Delete,
    Options,
    Patch,
}

#[derive(Debug, PartialEq)]
pub enum HttpParseError {
    InvalidMethod,
    InvalidPath,
    InvalidHttpVersion,

    Other(String),
}

#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    Http1_1,
    Http2_0,
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub http_version: HttpVersion,
}
