use crate::http::{RequestMethod, Status, StatusCode};
use crate::server::RequestHandler;
use std::collections::HashMap;
use std::sync::Arc;
use crate::http::Header;
use std::path::Path;
use std::env;
use crate::http::ContentType;
use crate::request::Request;
use crate::response::Response;


pub fn root_handler(req: Request) -> Result<Response, Response> {
    let body = "";
    let mut headers = HashMap::new();
    headers.insert(Header::ContentType, ContentType::TextPlain.to_string());
    headers.insert(Header::ContentLength, body.len().to_string());
    headers.extend(req.headers);
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        body.to_string(),
        headers,
    ))
}

pub fn echo_handler(req: Request) -> Result<Response, Response> {
    let body = req
        .target
        .strip_prefix("/echo")
        .and_then(|stripped| stripped.strip_prefix('/'))
        .map(|stripped| stripped.to_string())
        .unwrap_or_default();
    let mut headers = HashMap::new();
    headers.insert(Header::ContentType, ContentType::TextPlain.to_string());
    headers.insert(Header::ContentLength, body.len().to_string());
    headers.extend(req.headers);
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        body,
        headers,
    ))
}

pub fn user_agent_handler(req: Request) -> Result<Response, Response> {
    let body = req
        .headers
        .get(&Header::UserAgent)
        .cloned()
        .unwrap_or_else(|| "No User-Agent found".to_string());
    let mut headers = HashMap::new();
    headers.insert(Header::ContentType, ContentType::TextPlain.to_string());
    headers.insert(Header::ContentLength, body.len().to_string());
    headers.extend(req.headers);
    Ok(Response::builder(
        Status {
            code: StatusCode::Ok,
            message: "OK".to_string(),
        },
        body,
        headers,
    ))
}

pub fn files_handler(req: Request) -> Result<Response, Response> {
    let file_name = req
        .target
        .strip_prefix("/files")
        .and_then(|stripped| stripped.strip_prefix('/'))
        .map(|stripped| stripped.to_string())
        .unwrap_or_default();
    let filepath = format!(
        "{}{}",
        env::args().nth(2).expect("Argument missing"),
        file_name
    );
    let body = std::fs::read_to_string(filepath);
    let mut headers = HashMap::new();
    match body {
        Ok(body) => {
            headers.insert(Header::ContentLength, body.len().to_string());
            headers.insert(
                Header::ContentType,
                ContentType::ApplicationOctetStream.to_string(),
            );
            Ok(Response::builder(
                Status {
                    code: StatusCode::Ok,
                    message: "OK".to_string(),
                },
                body,
                headers,
            ))
        }
        Err(_) => Ok(Response::builder(
            Status {
                code: StatusCode::NotFound,
                message: "Not Found".to_string(),
            },
            "404 Not Found".to_string(),
            HashMap::new(),
        )),
    }
}

pub fn files_handler_create(req: Request) -> Result<Response, Response> {
    let file_name = req
        .target
        .strip_prefix("/files")
        .and_then(|stripped| stripped.strip_prefix('/'))
        .map(|stripped| stripped.to_string())
        .unwrap_or_default();
    let file = format!(
        "{}/{}",
        env::args().nth(2).expect("Argument missing"),
        file_name
    );
    let file_path = Path::new(&file);

    // Write the request body to the file
    if let Some(body) = &req.body {
        if let Err(_) = std::fs::write(file_path, body) {
            return Ok(Response::builder(
                Status {
                    code: StatusCode::InternalServerError,
                    message: "Internal Server Error".to_string(),
                },
                "500 Internal Server Error".to_string(),
                HashMap::new(),
            ));
        }
    }

    let mut headers = HashMap::new();
    headers.insert(Header::ContentType, ContentType::TextPlain.to_string());
    Ok(Response::builder(
        Status {
            code: StatusCode::Created,
            message: "Created".to_string(),
        },
        "".to_string(),
        headers,
    ))
}