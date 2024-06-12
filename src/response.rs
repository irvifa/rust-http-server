use std::collections::HashMap;
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
}

impl Response {
    pub fn builder(status: Status, body: String, headers: HashMap<String, String>) -> Response {
        Response {
            status,
            headers: headers,
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

        // Write each header
        for (key, value) in &self.headers {
            write!(&mut buffer, "{}: {}\r\n", key, value).unwrap();
        }

        // End headers section
        buffer.extend_from_slice(b"\r\n");

        // Append the body
        buffer.extend_from_slice(&self.body);

        buffer
    }

    pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        stream.write_all(&self.to_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
