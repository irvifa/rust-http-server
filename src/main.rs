use std::{
    io::{Write},
    net::{TcpListener, TcpStream},
    collections::{HashMap},
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
                    message: "OK".to_string(),
                },
                "".to_string(),
                HashMap::new(),
            );
            response.send(&mut stream).unwrap();
        }
        "/echo" => {
            let body = req.body.clone().unwrap();
            let response = Response::builder(
                Status {
                    code: StatusCode::Ok,
                    message: "OK".to_string(),
                },
                body,
                req.headers,
            );
            response.send(&mut stream).unwrap();
        }
        "/user-agent" => {
            let body = req.headers.get("User-Agent").cloned().unwrap_or_else(|| "No User-Agent found".to_string());
            let response = Response::builder(
                Status {
                    code: StatusCode::Ok,
                    message: "OK".to_string(),
                },
                body,
                req.headers,
            );
            response.send(&mut stream).unwrap();
        }
        _ => {
            let response = Response::builder(
                Status {
                    code: StatusCode::NotFound,
                    message: "Not Found".to_string(),
                },
                "".to_string(),
                HashMap::new(),
            );
            response.send(&mut stream).unwrap();
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
