mod encoding;
mod request;
mod response;
mod routes;
mod server;
mod http;
mod router;

use crate::routes::{root_handler, echo_handler, user_agent_handler, files_handler, files_handler_create};
use crate::server::{HttpServer};
use crate::http::{RequestMethod};
use crate::router::Router;
use std::sync::{Arc, RwLock};
use std::net::TcpListener;

fn main() {
    let mut router = Router::new();
    let router_arc = Arc::new(RwLock::new(router));

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc
            .write()
            .unwrap()
            .add_route(RequestMethod::GET, "/", root_handler);
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc
            .write()
            .unwrap()
            .add_route(RequestMethod::GET, "/echo", echo_handler);
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc
            .write()
            .unwrap()
            .add_route(RequestMethod::GET, "/user-agent", user_agent_handler);
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc
            .write()
            .unwrap()
            .add_route(RequestMethod::GET, "/files", files_handler);
    }

    {
        let router_arc_clone = Arc::clone(&router_arc);
        router_arc
            .write()
            .unwrap()
            .add_route(RequestMethod::POST, "/files", files_handler_create);
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
