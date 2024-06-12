use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};

type RequestHandler = fn(&str, &HashMap<String, String>, &str) -> String;

pub struct Router {
    routes: HashMap<String, RequestHandler>,
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, path: &str, handler: RequestHandler) {
        self.routes.insert(path.to_string(), handler);
    }

    fn route(
        &self,
        method: &RequestMethod,
        path: &str,
        headers: &HashMap<String, String>,
        body: &str) -> Result<Response, Response> {
        if let Some(handler) = self.routes.get(path) {
            let request = Request {
                method: method,
                target: path.to_string(),
                version: "HTTP/1.1".to_string(),
                headers: headers.clone(),
                body: Some(body.to_string()),
            };
            handler(request)
        } else {
            Err(Response::builder(
                Status {
                    code: StatusCode::NotFound,
                    message: "Not Found".to_string(),
                },
                "404 Not Found".to_string(),
                HashMap::new(),
            ))
        }
    }
}
