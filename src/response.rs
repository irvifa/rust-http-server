use crate::encoding::{ContentEncoding, Encoding};
use hex::encode;
use std::collections::{HashMap, HashSet};
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
}

pub struct Status {
    pub code: StatusCode,
    pub message: String,
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
            encoded_body =
                ContentEncoding::GZIP.encode(&String::from_utf8(self.body.clone()).unwrap());
            // encoded_body = encode(&encoded_body).into();
            // Update Content-Encoding header
            headers.insert("Content-Encoding".to_string(), "gzip".to_string());
        }

        // Update Content-Length header to reflect the encoded body length
        headers.insert("Content-Length".to_string(), encoded_body.len().to_string());

        // Write each header
        for (key, value) in &headers {
            write!(&mut buffer, "{}: {}\r\n", key, value).unwrap();
        }

        // End headers section
        buffer.extend_from_slice(b"\r\n");

        // Append the encoded body
        buffer.extend_from_slice(&encoded_body);

        buffer
    }

    pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let response_bytes = self.to_bytes();
        stream.write_all(&response_bytes)?;
        stream.flush()?;
        Ok(())
    }
}
