use std::{
    io::{Write},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    sync::{Arc, RwLock},
};

use request::{RequestMethod, Request};
mod request;

use response::{Response, Status, StatusCode};
mod response;

use router::Router;
mod router;

use server::{HttpServer, RequestHandler};
mod server;

fn parse_echo_body(request: &Request) -> Option<String> {
    request.target.strip_prefix("/echo").and_then(|stripped| stripped.strip_prefix('/')).map(|stripped| stripped.to_string())
}

fn parse_user_agent_body(request: &Request) -> Option<String> {
    request.headers.get("User-Agent").cloned()
}


fn root_handler(req: Request) -> Result<Response, Response> {
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        "".to_string(),
        HashMap::new(),
    ))
}

fn echo_handler(req: Request) -> Result<Response, Response> {
    println!("{}", req.body.clone().unwrap_or_default());
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        req.body.unwrap_or_default(),
        req.headers,
    ))
}

fn user_agent_handler(req: Request) -> Result<Response, Response> {
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        req.body.unwrap_or_default(),
        req.headers,
    ))
}


fn main() {
    let mut router = Router::new();
    let router_arc = Arc::new(RwLock::new(router));

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc.write().unwrap().add_route("/", |req| root_handler(req), |_req| None);
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc.write().unwrap().add_route("/echo", |req| echo_handler(req), parse_echo_body);
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc.write().unwrap().add_route("/user-agent", |req| user_agent_handler(req), parse_user_agent_body);
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

