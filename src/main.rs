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

fn echo_handler(router: Arc<RwLock<Router>>, req: Request) -> Result<Response, Response> {
    let mut req = req;
    let router = router.read().unwrap();
    req.body = router.parse_body(&req);
    req.target = router.parse_target(req.target.to_string());
    println!("{}", req.body.clone().unwrap());
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        req.body.unwrap(),
        req.headers,
    ))
}

fn user_agent_handler(router: Arc<RwLock<Router>>, req: Request) -> Result<Response, Response> {
    let mut req = req;
    let router = router.read().unwrap();
    req.body = router.parse_body(&req);
    req.target = router.parse_target(req.target.to_string());
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        req.body.unwrap(),
        req.headers,
    ))
}

fn main() {
    let mut router = Router::new();
    let router_arc = Arc::new(RwLock::new(router));

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc.write().unwrap().add_route("/", |req| root_handler(req));
    }
    
    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc.write().unwrap().add_route("/echo", move |req| echo_handler(router_arc_clone.clone(), req));
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc.write().unwrap().add_route("/user-agent", move |req| user_agent_handler(router_arc_clone.clone(), req));
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
