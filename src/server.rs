use crate::http::{Status, StatusCode};
use crate::request::Request;
use crate::response::Response;
use crate::router::Router;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};

pub type RequestHandler = Arc<dyn Fn(Request) -> Result<Response, Response> + Send + Sync>;

pub struct HttpServer {
    router: Arc<RwLock<Router>>,
}

impl HttpServer {
    pub fn new(router: Arc<RwLock<Router>>) -> Self {
        HttpServer { router }
    }

    pub fn handle_client(&self, mut stream: TcpStream) {
        let req = match Request::builder(&stream) {
            Ok(req) => req,
            Err(_) => {
                let response = Response::builder(
                    Status::new(StatusCode::NotFound),
                    "Bad Request".to_string(),
                    HashMap::new(),
                );
                response.send(&mut stream).unwrap();
                return;
            }
        };

        let response = match self.router.read().unwrap().route(req) {
            Ok(response) => response,
            Err(response) => response,
        };

        response.send(&mut stream).unwrap();
    }
}
