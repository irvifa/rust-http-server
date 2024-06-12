use std::{
    io::{Write},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    sync::{Arc, RwLock},
};
use crate::request::Request;
use crate::response::{Response, Status, StatusCode};
use crate::server::RequestHandler;

pub struct Router {
    routes: HashMap<String, RequestHandler>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request) -> Result<Response, Response> + Send + Sync + 'static,
    {
        self.routes.insert(path.to_string(), Arc::new(handler));
    }

    pub fn route(&self, req: Request) -> Result<Response, Response> {
        if let Some(handler) = self.routes.get(req.target.as_str()) {
            handler(req)
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

    pub fn get_prefixes(&self) -> Vec<String> {
        self.routes.keys().cloned().collect()
    }

    fn find_prefix<'a>(target: &'a str, prefixes: &'a [String]) -> Option<&'a str> {
        prefixes
            .iter()
            .find(|&&ref prefix| target.starts_with(prefix))
            .map(|prefix| prefix.as_str())
    }

    pub fn parse_target(&self, path: String) -> String {
        let prefixes = self.get_prefixes();
        match Self::find_prefix(&path, &prefixes) {
            Some(prefix) => prefix.to_string(),
            None => path,
        }
    }

    pub fn parse_body(&self, request: &Request) -> Option<String> {
        let prefixes = self.get_prefixes();
        let path = self.parse_target(request.target.to_string());
        match Self::find_prefix(&path, &prefixes) {
            Some(prefix) => match prefix {
                "/echo" => {
                    let body = request.target.strip_prefix(prefix).and_then(|stripped| stripped.strip_prefix('/')).map(|stripped| stripped.to_string());
                    body.clone()
                },
                "/user-agent" => request.headers.get("User-Agent").cloned(),
                _ => None,
            },
            None => None,
        }
    }
}
