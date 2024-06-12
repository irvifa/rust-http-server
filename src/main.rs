use std::{
    io::{Write},
    net::{TcpListener, TcpStream},
    collections::{HashMap},
};

use request::{RequestMethod, Request};
mod request;

use response::{Response, Status, StatusCode};
mod response;

fn get_prefixes() -> Vec<String> {
    vec!["/echo".to_string(), "/user-agent".to_string()]
}

fn find_prefix<'a>(target: &'a str, prefixes: &'a [String]) -> Option<&'a str> {
    prefixes
        .iter()
        .find(|&&ref prefix| target.starts_with(prefix))
        .map(|prefix| prefix.as_str())
}

fn parse_target(path: String) -> String {
    let prefixes = get_prefixes();
    match find_prefix(&path, &prefixes) {
        Some(prefix) => prefix.to_string(),
        None => path,
    }
}

fn root_handler(_req: Request) -> Result<Response, Response> {
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        "".to_string(),
        HashMap::new(),
    ))
}

fn echo_handler(mut req: Request) -> Result<Response, Response> {
    req.body = parse_body(&req);
    req.target = parse_target(req.target.to_string());
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        req.body.unwrap(),
        req.headers,
    ))
}


fn user_agent_handler(mut req: Request) -> Result<Response, Response> {
    req.body = parse_body(&req);
    req.target = parse_target(req.target.to_string());
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        req.body.unwrap(),
        req.headers,
    ))
}

fn parse_body(req: &Request) -> Option<String> {
    let prefixes = get_prefixes();
    let path = parse_target(req.target.to_string());
    match find_prefix(&path, &prefixes) {
        Some(prefix) => match prefix {
            "/echo" => {
                let body = req.target.strip_prefix(prefix).and_then(|stripped| stripped.strip_prefix('/')).map(|stripped| stripped.to_string());
                body.clone()
            },
            "/user-agent" => req.headers.get("User-Agent").cloned(),
            _ => None,
        },
        None => None,
    }
}

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
                std::thread::spawn(||request_handler(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
