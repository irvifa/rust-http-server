use crate::encoding::{ContentEncoding, Encoding};
use hex::encode;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::Write;
use std::net::TcpStream;

pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
    Created = 201,
    InternalServerError = 500,
    BadRequest = 400,
}

impl StatusCode {
    fn to_u16(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::NotFound => 404,
            StatusCode::Created => 201,
            StatusCode::InternalServerError => 500,
            StatusCode::BadRequest => 400,
        }
    }

    fn to_reason_phrase(&self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "Not Found",
            StatusCode::Created => "Created",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::BadRequest => "Bad Request",
        }
    }

    fn to_string(&self) -> String {
        format!("{} {}", self.to_u16(), self.to_reason_phrase())
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct Status {
    pub code: StatusCode,
    pub message: String,
}

impl Status {
    pub fn new(code: StatusCode) -> Self {
        Status {
            message: code.to_reason_phrase().to_string(),
            code,
        }
    }
}

pub struct Response {
    pub version: String,
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub content_encodings: HashSet<ContentEncoding>,
}

impl Response {
    pub fn builder(status: Status, body: String, headers: HashMap<String, String>) -> Response {
        let mut content_encodings = HashSet::new();

        if let Some(accept_encoding) = headers.get("Accept-Encoding") {
            for encoding in accept_encoding.split(',').map(|s| s.trim()) {
                content_encodings.insert(ContentEncoding::from_string(encoding));
            }
        }

        Response {
            status,
            headers,
            content_encodings,
            version: "HTTP/1.1".to_string(),
            body: body.into_bytes(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        write!(
            &mut buffer,
            "{} {} {}\r\n",
            self.version,
            self.status.code.to_u16(),
            self.status.message
        )
        .unwrap();

        let mut encoded_body = self.body.clone();
        let mut headers = self.headers.clone();

        // Encode the body if gzip is present in content encodings
        if self.content_encodings.contains(&ContentEncoding::GZIP) {
            println!("Encoding body with GZIP");
            encoded_body =
                ContentEncoding::GZIP.encode(&String::from_utf8(self.body.clone()).unwrap());

            // Update Content-Encoding header
            headers.insert("Content-Encoding".to_string(), "gzip".to_string());
        }

        // Update Content-Length header to reflect the encoded body length
        headers.insert("Content-Length".to_string(), encoded_body.len().to_string());

        // Write each header
        println!("Headers:");
        for (key, value) in &headers {
            println!("{}: {}", key, value);
            write!(&mut buffer, "{}: {}\r\n", key, value).unwrap();
        }

        // End headers section
        buffer.extend_from_slice(b"\r\n");

        // Append the encoded body
        buffer.extend_from_slice(&encoded_body);

        println!(
            "Body before encoding: {:?}",
            String::from_utf8_lossy(&self.body)
        );
        println!("Body after encoding: {:?}", encode(&encoded_body));
        println!("Response bytes: {:?}", encode(&buffer));

        buffer
    }

    pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let response_bytes = self.to_bytes();
        println!("Response: {:?}", String::from_utf8_lossy(&response_bytes));
        stream.write_all(&response_bytes)?;
        stream.flush()?;
        Ok(())
    }
}
