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
