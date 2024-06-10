use std::{
    io::{BufRead, BufReader, ErrorKind, Error},
    net::{TcpStream},
};

pub enum RequestMethod {
    GET,
    POST,
    PUT
}

pub struct Request {
    pub method: RequestMethod,
    pub target: String,
    pub version: String,
    pub body: Option<String>,
}

// https://doc.rust-lang.org/rust-by-example/fn/methods.html
impl Request {
    fn get_prefixes() -> Vec<String> {
        vec!["/echo".to_string()]
    }

    fn find_prefix<'a>(target: &'a str, prefixes: &'a Vec<String>) -> Option<&'a str> {
        prefixes.iter().find(|&&ref prefix| target.starts_with(prefix)).map(|prefix| prefix.as_str())
    }

    fn parse_target(path: String) -> String {
        let prefixes = Self::get_prefixes();
        match Self::find_prefix(&path, &prefixes) {
            Some(prefix) => prefix.to_string(),
            None => path,
        }
    }

    fn parse_body(path: String) -> Option<String> {
        let prefixes = Self::get_prefixes();
        match Self::find_prefix(&path, &prefixes) {
            Some(prefix) => path.strip_prefix(prefix).and_then(|stripped| stripped.strip_prefix('/')).map(|stripped| stripped.to_string()),
            None => None,
        }
    }

    pub fn builder(stream: &TcpStream) -> Result<Request, Error> {
        let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
        let mut request_str = String::new();
        let _num_read = buf_reader.read_line(&mut request_str).unwrap();
        let _headers: Vec<String> = buf_reader
            .lines()
            .map(|l| l.unwrap().trim_end().to_string())
            .take_while(|l| l.len() > 0)
            .collect();
        let parts: Vec<&str> = request_str.split(' ').collect();
        if parts.len() == 3 {
            let [method, target, version] = match parts.as_slice() {
                [method, target, version] => [method, target, version],
                _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid request.".to_string())),
            };
            Ok(Request {
                method: Self::method_from_string(method)?,
                target: Self::parse_target(target.to_string()),
                version: version.to_string(),
                body: Self::parse_body(target.to_string()),
            })
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
