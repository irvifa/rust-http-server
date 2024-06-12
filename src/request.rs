use std::{
    io::{BufRead, BufReader, ErrorKind, Error},
    net::TcpStream,
    collections::HashMap,
};

pub enum RequestMethod {
    GET,
    POST,
    PUT,
}

pub struct Request {
    pub method: RequestMethod,
    pub target: String,
    pub version: String,
    pub body: Option<String>,
    pub headers: HashMap<String, String>,
}

impl Request {
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
        let prefixes = Self::get_prefixes();
        match Self::find_prefix(&path, &prefixes) {
            Some(prefix) => prefix.to_string(),
            None => path,
        }
    }

    fn parse_body(request: &Request) -> Option<String> {
        let prefixes = Self::get_prefixes();
        let path = Self::parse_target(request.target.to_string());
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

    pub fn builder(stream: &TcpStream) -> Result<Request, Error> {
        let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
        let mut request_str = String::new();
        buf_reader.read_line(&mut request_str)?;

        let headers: HashMap<String, String> = buf_reader
            .lines()
            .filter_map(|l| l.ok())
            .map(|line| line.trim_end().to_string())
            .take_while(|line| !line.is_empty())
            .filter_map(|line| {
                let mut parts = line.splitn(2, ':');
                let key = parts.next()?.trim().to_string();
                let value = parts.next().unwrap_or("").trim().to_string();
                Some((key, value))
            })
            .collect();

        let parts: Vec<&str> = request_str.split(' ').collect();
        if parts.len() == 3 {
            let [method, target, version] = match parts.as_slice() {
                [method, target, version] => [method, target, version],
                _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid request.".to_string())),
            };

            let mut request = Request {
                method: Self::method_from_string(method)?,
                target: target.to_string(),
                version: version.to_string(),
                body: None,
                headers,
            };
            request.body = Self::parse_body(&request);
            request.target = Self::parse_target(target.to_string());
            Ok(request)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid request.".to_string()))
        }
    }

    pub fn method_from_string(method_as_str: &str) -> Result<RequestMethod, Error> {
        match method_as_str {
            "GET" => Ok(RequestMethod::GET),
            "POST" => Ok(RequestMethod::POST),
            "PUT" => Ok(RequestMethod::PUT),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid method: {method_as_str}"),
            )),
        }
    }
}
