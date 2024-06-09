// Uncomment this block to pass the first stage
use std::{
    io::{BufRead, BufReader, Write, ErrorKind, Error},
    net::{TcpListener, TcpStream},
};

enum RequestMethod {
    GET,
    POST,
    PUT
}

struct Request {
    method: String,
    target: String,
    version: String,
    body: Option<String>,
}

// https://doc.rust-lang.org/rust-by-example/fn/methods.html
impl Request {
    fn builder(&self, stream: &TcpStream) -> Result<Request, E> {
        let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
        let mut request_str = String::new();
        let _num_read = buf_reader.read_line(&mut request_str).unwrap();
        let headers: Vec<String> = buf_reader
            .lines()
            .map(|l| l.unwrap().trim_end().to_string())
            .take_while(|l| l.len() > 0)
            .collect();
        let parts: Vec<&str> = request_str.split(' ').collect();
        if let [method, target, version] = parts.as_slice() {
            Result::Ok(Request {
                method: self.method_from_string(method.to_string()),
                target: target.to_string(),
                version: version.to_string(),
                body: None,
            });
        } else {
            Result::Err("Invalid request.".to_string())
        }
    }

    fn method_from_string(method_as_str: &str) -> Result<RequestMethod> {
        match method_as_str {
            "GET" => RequestMethod::GET,
            "POST" => RequestMethod::POST,
            "PUT" => RequestMethod::PUT,
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid method: {method_as_str}"),
            )),
        }
    }
}

fn request_handler(mut stream: TcpStream) {
    let req = Request::builder(&stream).unwrap();
    match req.target.as_str() {
        "/" => stream
            .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
            .unwrap(),
        _ => stream
            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
            .unwrap(),
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
