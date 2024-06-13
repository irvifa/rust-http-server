use crate::request::{Request, RequestMethod};
use crate::response::{Response, Status, StatusCode};
use crate::server::RequestHandler;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Router {
    routes: HashMap<
        (RequestMethod, String),
        Arc<dyn Fn(Request) -> Result<Response, Response> + Send + Sync>,
    >,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(&mut self, method: RequestMethod, path: &str, handler: F)
    where
        F: Fn(Request) -> Result<Response, Response> + Send + Sync + 'static,
    {
        self.routes
            .insert((method, path.to_string()), Arc::new(handler));
    }

    pub fn route(&self, req: Request) -> Result<Response, Response> {
        let (method, key) = (req.method.clone(), req.target.clone());
        if let Some(handler) = self.contains_prefix(&method, &key) {
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
        self.routes.keys().map(|(_, path)| path.clone()).collect()
    }

    pub fn contains_prefix(&self, method: &RequestMethod, prefix: &str) -> Option<&RequestHandler> {
        println!("{}", prefix);
        self.routes.iter().find_map(|((_method, key), handler)| {
            if key == "/" && prefix == "/" && method == _method {
                Some(handler)
            } else if prefix.starts_with(key) && key != "/" && method == _method {
                Some(handler)
            } else {
                None
            }
        })
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
}
