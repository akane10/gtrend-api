use rocket::response::{self, Responder};
use rocket::{http::Status, request::Request};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    GtrendError(gtrend::Error),
    StatusError(Status),
    JsonError(serde_json::Error),
    IoError(std::io::Error),
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        println!("Oppss: {:#?}", self);
        match self {
            Error::StatusError(e) => Err(e),
            _ => Err(Status::InternalServerError),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::GtrendError(ref x) => write!(f, "{:#?}", x),
            Error::StatusError(ref x) => write!(f, "{}", x),
            Error::JsonError(ref x) => write!(f, "{:#?}", x),
            Error::IoError(ref x) => write!(f, "{:#?}", x),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! error_wrap {
    ($f:ty, $e:expr) => {
        impl From<$f> for Error {
            fn from(f: $f) -> Error {
                $e(f)
            }
        }
    };
}

error_wrap!(gtrend::Error, Error::GtrendError);
error_wrap!(serde_json::Error, Error::JsonError);
error_wrap!(Status, Error::StatusError);
error_wrap!(std::io::Error, Error::IoError);
