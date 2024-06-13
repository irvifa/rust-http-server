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
