extern crate flate2;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Write};

pub enum ContentEncoding {
    NONE,
    GZIP,
}

pub trait Encoding {
    fn from_string(encoding: &str) -> ContentEncoding;
    fn to_string(encoding: &ContentEncoding) -> Option<String>;
    fn encode(&self, input: &str) -> Vec<u8>;
    fn decode(&self, input: &[u8]) -> String;
}

impl Encoding for ContentEncoding {
    fn from_string(encoding: &str) -> ContentEncoding {
        match encoding {
            "gzip" => ContentEncoding::GZIP,
            "none" => ContentEncoding::NONE,
            _ => ContentEncoding::NONE,
        }
    }

    fn to_string(encoding: &ContentEncoding) -> Option<String> {
        match encoding {
            ContentEncoding::GZIP => Some("gzip".to_string()),
            _ => None,
        }
    }

    fn encode(&self, input: &str) -> Vec<u8> {
        match self {
            ContentEncoding::NONE => input.as_bytes().to_vec(),
            ContentEncoding::GZIP => {
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(input.as_bytes()).unwrap();
                encoder.finish().unwrap()
            }
        }
    }

    fn decode(&self, input: &[u8]) -> String {
        match self {
            ContentEncoding::NONE => String::from_utf8(input.to_vec()).unwrap(),
            ContentEncoding::GZIP => {
                let mut decoder = GzDecoder::new(input);
                let mut decoded = String::new();
                decoder.read_to_string(&mut decoded).unwrap();
                decoded
            }
        }
    }
}
