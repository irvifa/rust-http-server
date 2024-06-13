mod encoding;
mod request;
mod response;
mod router;
mod server;

use crate::encoding::{ContentEncoding, Encoding};
use crate::request::{Request, RequestMethod};
use crate::response::{Response, Status, StatusCode};
use crate::router::Router;
use crate::server::{HttpServer, RequestHandler};
use std::{
    collections::HashMap,
    env,
    io::Write,
    net::{TcpListener, TcpStream},
    path::Path,
    sync::{Arc, RwLock},
};

fn root_handler(req: Request) -> Result<Response, Response> {
    let body = "";
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    headers.insert("Content-Length".to_string(), body.len().to_string());
    headers.extend(req.headers);
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        body.to_string(),
        headers,
    ))
}

fn echo_handler(req: Request) -> Result<Response, Response> {
    let body = req
        .target
        .strip_prefix("/echo")
        .and_then(|stripped| stripped.strip_prefix('/'))
        .map(|stripped| stripped.to_string())
        .unwrap_or_default();
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    headers.insert("Content-Length".to_string(), body.len().to_string());
    println!("{}", body);
    headers.extend(req.headers);
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        body,
        headers,
    ))
}

fn user_agent_handler(req: Request) -> Result<Response, Response> {
    let body = req
        .headers
        .get("User-Agent")
        .cloned()
        .unwrap_or_else(|| "No User-Agent found".to_string());
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    headers.insert("Content-Length".to_string(), body.len().to_string());
    headers.extend(req.headers);
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        body,
        headers,
    ))
}

fn files_handler(req: Request) -> Result<Response, Response> {
    let file_name = req
        .target
        .strip_prefix("/files")
        .and_then(|stripped| stripped.strip_prefix('/'))
        .map(|stripped| stripped.to_string())
        .unwrap_or_default();
    let filepath = format!(
        "{}{}",
        env::args().nth(2).expect("Argument missing"),
        file_name
    );
    let body = std::fs::read_to_string(filepath);
    let mut headers = HashMap::new();
    match body {
        Ok(body) => {
            headers.insert("Content-Length".to_string(), body.len().to_string());
            headers.insert(
                "Content-Type".to_string(),
                "application/octet-stream".to_string(),
            );
            Ok(Response::builder(
                Status {
                    code: StatusCode::Ok,
                    message: "OK".to_string(),
                },
                body,
                headers,
            ))
        }
        Err(_) => Ok(Response::builder(
            Status {
                code: StatusCode::NotFound,
                message: "Not Found".to_string(),
            },
            "404 Not Found".to_string(),
            HashMap::new(),
        )),
    }
}

fn files_handler_create(req: Request) -> Result<Response, Response> {
    let file_name = req
        .target
        .strip_prefix("/files")
        .and_then(|stripped| stripped.strip_prefix('/'))
        .map(|stripped| stripped.to_string())
        .unwrap_or_default();
    let file = format!(
        "{}/{}",
        env::args().nth(2).expect("Argument missing"),
        file_name
    );
    let file_path = Path::new(&file);

    // Write the request body to the file
    if let Some(body) = &req.body {
        if let Err(_) = std::fs::write(file_path, body) {
            return Ok(Response::builder(
                Status {
                    code: StatusCode::InternalServerError,
                    message: "Internal Server Error".to_string(),
                },
                "500 Internal Server Error".to_string(),
                HashMap::new(),
            ));
        }
    }

    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "text/plain".to_string());
    headers.insert("Content-Length".to_string(), "0".to_string());
    Ok(Response::builder(
        Status {
            code: StatusCode::Created,
            message: "Created".to_string(),
        },
        "".to_string(),
        headers,
    ))
}

fn main() {
    let mut router = Router::new();
    let router_arc = Arc::new(RwLock::new(router));

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc
            .write()
            .unwrap()
            .add_route(RequestMethod::GET, "/", |req| root_handler(req));
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc
            .write()
            .unwrap()
            .add_route(RequestMethod::GET, "/echo", |req| echo_handler(req));
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc
            .write()
            .unwrap()
            .add_route(RequestMethod::GET, "/user-agent", |req| {
                user_agent_handler(req)
            });
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc
            .write()
            .unwrap()
            .add_route(RequestMethod::GET, "/files", |req| files_handler(req));
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc
            .write()
            .unwrap()
            .add_route(RequestMethod::POST, "/files", |req| {
                files_handler_create(req)
            });
    }

    let server = Arc::new(HttpServer::new(router_arc.clone())); // Wrap HttpServer in Arc

    // Start the server
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Listening on http://127.0.0.1:4221");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let server = Arc::clone(&server); // Clone the Arc reference
                std::thread::spawn(move || {
                    server.handle_client(stream); // Use the cloned server
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
