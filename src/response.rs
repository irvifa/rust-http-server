use crate::encoding::{ContentEncoding, Encoding};
use std::collections::HashMap;
use std::collections::HashSet;
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
        println!("{}", self.version);
        write!(
            &mut buffer,
            "{} {} {}\r\n",
            self.version,
            self.status.code.to_u16(),
            self.status.message
        )
        .unwrap();

        // Write each header
        for (key, value) in &self.headers {
            write!(&mut buffer, "{}: {}\r\n", key, value).unwrap();
        }

        // Write Content-Encoding header if any encoding is present
        if !self.content_encodings.is_empty() {
            let encoding_str = self
                .content_encodings
                .iter()
                .filter_map(|e| ContentEncoding::to_string(e))
                .collect::<Vec<String>>()
                .join(", ");
            write!(&mut buffer, "Content-Encoding: {}\r\n", encoding_str).unwrap();
        }

        // End headers section
        buffer.extend_from_slice(b"\r\n");

        buffer.extend_from_slice(&self.body);

        buffer
    }

    pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        stream.write_all(&self.to_bytes())?;
        stream.flush()?;
        Ok(())
    }

    pub fn print_encodings(&self) {
        for encoding in &self.content_encodings {
            println!(
                "{}",
                ContentEncoding::to_string(encoding).unwrap_or_else(|| "none".to_string())
            );
        }
    }
}
