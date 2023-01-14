use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::parse::parse;
use crate::types::Server;

impl Server {
    pub fn new(on: &str) -> Server {
        Server {
            listener: TcpListener::bind(on).unwrap(),
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        thread::spawn(move || match parse(&mut stream) {
            Ok(_) => {
                // TODO: create and use a Response struct
                match stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\nok") {
                    Ok(_) => println!("Response sent"),
                    Err(e) => println!("Error: {:?}", e),
                }
            }
            Err(err) => println!("Error: {:?}", err),
        });
    }

    pub fn listen_once(&mut self) {
        match self.listener.accept() {
            Ok((stream, _)) => Server::handle_connection(stream),
            Err(err) => println!("Error: {:?}", err),
        }
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => Server::handle_connection(stream),
                Err(err) => println!("Error: {:?}", err),
            }
        }
    }
}
