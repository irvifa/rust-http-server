use crate::http::{Header, RequestMethod};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::net::TcpStream;
use std::io::Read;

pub struct Request {
    pub method: RequestMethod,
    pub target: String,
    pub version: String,
    pub body: Option<String>,
    pub headers: HashMap<Header, String>,
}

impl Request {
    pub fn builder(stream: &TcpStream) -> Result<Request, Error> {
        let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
        let mut request_str = String::new();
        buf_reader.read_line(&mut request_str)?;

        let mut headers = HashMap::new();
        let mut line = String::new();

        // Read headers
        while buf_reader.read_line(&mut line)? > 0 {
            let trimmed_line = line.trim().to_string();
            if trimmed_line.is_empty() {
                break;
            }
            if let Some((key, value)) = trimmed_line.split_once(':') {
                headers.insert(Header::from_string(key.trim()), value.trim().to_string());
            }
            line.clear();
        }

        let parts: Vec<&str> = request_str.split(' ').collect();
        if parts.len() == 3 {
            let [method, target, version] = match parts.as_slice() {
                [method, target, version] => [method, target, version],
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Invalid request.".to_string(),
                    ))
                }
            };

            let method = RequestMethod::from_string(method)?;

            // Check for Content-Length header to read the body
            let mut body = String::new();
            if let Some(content_length) = headers.get(&Header::ContentLength) {
                let content_length: usize = content_length.parse().unwrap_or(0);
                if content_length > 0 {
                    let mut buffer = vec![0; content_length];
                    buf_reader.read_exact(&mut buffer)?;
                    body = String::from_utf8(buffer).unwrap_or_default();
                }
            }

            let request = Request {
                method,
                target: target.to_string(),
                version: version.to_string(),
                body: if body.is_empty() { None } else { Some(body) },
                headers,
            };

            Ok(request)
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid request.".to_string(),
            ))
        }
    }
}