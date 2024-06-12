use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    sync::{Arc, RwLock},
};

use request::{Request, RequestMethod};
mod request;

use response::{Response, Status, StatusCode};
mod response;

use router::Router;
mod router;

struct HttpServer {
    router: Arc<RwLock<Router>>,
}

impl HttpServer {
    fn new(router: Arc<RwLock<Router>>) -> Self {
        HttpServer { router }
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let req = match Request::builder(&stream) {
            Ok(req) => req,
            Err(_) => {
                let response = Response::builder(
                    Status {
                        code: StatusCode::NotFound,
                        message: "Bad Request".to_string(),
                    },
                    "Bad Request".to_string(),
                    HashMap::new(),
                );
                response.send(&mut stream).unwrap();
                return;
            }
        };
        let response = self.router.read().unwrap().route(req);
        response.send(&mut stream).unwrap();
    }
}
