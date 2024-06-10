// Uncomment this block to pass the first stage
use std::{
    io::{Write},
    net::{TcpListener, TcpStream},
};

use request::{RequestMethod, Request};
mod request;

use response::{Response, Status, StatusCode};
mod response;

fn request_handler(mut stream: TcpStream) {
    let req = Request::builder(&stream).unwrap();
    match req.target.as_str() {
        "/" => {
            let response = Response::builder(
                Status {
                    code: StatusCode::Ok,
                    message: "OK".to_string().into_bytes(),
                },
                "".to_string(),
            );
            stream.write_all(&response.to_bytes()).unwrap();
        }
        "/echo" => {
            let body = req.body.unwrap();
            let response = Response::builder(
                Status {
                    code: StatusCode::Ok,
                    message: "OK".to_string().into_bytes(),
                },
                body,
            );
            stream.write_all(&response.to_bytes()).unwrap();
        }
        _ => {
            let response = Response::builder(
                Status {
                    code: StatusCode::NotFound,
                    message: "Not Found".to_string().into_bytes(),
                },
                "".to_string(),
            );
            stream.write_all(&response.to_bytes()).unwrap();
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                request_handler(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
