use std::fmt;
use std::io::Error;
use std::io::ErrorKind;
use std::io::{Read, Write};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
}

impl RequestMethod {
    pub fn from_string(method: &str) -> Result<Self, Error> {
        match method {
            "GET" => Ok(RequestMethod::GET),
            "POST" => Ok(RequestMethod::POST),
            "PUT" => Ok(RequestMethod::PUT),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid method: {}", method),
            )),
        }
    }
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestMethod::GET => write!(f, "GET"),
            RequestMethod::POST => write!(f, "POST"),
            RequestMethod::PUT => write!(f, "PUT"),
        }
    }
}

pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
    Created = 201,
    InternalServerError = 500,
    BadRequest = 400,
}

impl StatusCode {
    pub fn to_u16(&self) -> u16 {
        match self {
            StatusCode::Ok => 200,
            StatusCode::NotFound => 404,
            StatusCode::Created => 201,
            StatusCode::InternalServerError => 500,
            StatusCode::BadRequest => 400,
        }
    }

    pub fn to_reason_phrase(&self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::NotFound => "Not Found",
            StatusCode::Created => "Created",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::BadRequest => "Bad Request",
        }
    }

    pub fn to_string(&self) -> String {
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Header {
    ContentLength,
    ContentType,
    AcceptEncoding,
    ContentEncoding,
    UserAgent,
    Host,
    Accept,
    Custom(String),
}

impl Header {
    pub fn from_string(header: &str) -> Self {
        match header.to_lowercase().as_str() {
            "content-length" => Header::ContentLength,
            "content-type" => Header::ContentType,
            "accept-encoding" => Header::AcceptEncoding,
            "content-encoding" => Header::ContentEncoding,
            "user-agent" => Header::UserAgent,
            "host" => Header::Host,
            "accept" => Header::Accept,
            _ => Header::Custom(header.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Header::ContentLength => "Content-Length".to_string(),
            Header::ContentType => "Content-Type".to_string(),
            Header::AcceptEncoding => "Accept-Encoding".to_string(),
            Header::ContentEncoding => "Content-Encoding".to_string(),
            Header::UserAgent => "User-Agent".to_string(),
            Header::Host => "Host".to_string(),
            Header::Accept => "Accept".to_string(),
            Header::Custom(value) => value.clone(),
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContentType {
    TextPlain,
    ApplicationOctetStream,
    Custom(String),
}

impl ContentType {
    pub fn from_string(content_type: &str) -> Self {
        match content_type.to_lowercase().as_str() {
            "text/plain" => ContentType::TextPlain,
            "application/octet-stream" => ContentType::ApplicationOctetStream,
            _ => ContentType::Custom(content_type.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ContentType::TextPlain => "text/plain".to_string(),
            ContentType::ApplicationOctetStream => "application/octet-stream".to_string(),
            ContentType::Custom(value) => value.clone(),
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
