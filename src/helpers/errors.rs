use serde::export::Formatter;
use std::fmt;
use std::fmt::Debug;
use std::{error, result};

#[derive(Debug)]
pub enum DatasheepError {
    InternalServer,
    BadRequest,
    Unauthorised,
}

impl fmt::Display for DatasheepError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            DatasheepError::InternalServer => write!(f, "Internal Server Error"),
            DatasheepError::BadRequest => write!(f, "Bad Request"),
            DatasheepError::Unauthorised => write!(f, "Unauthorised"),
        }
    }
}

impl error::Error for DatasheepError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DatasheepError::InternalServer => None,
            DatasheepError::BadRequest => None,
            DatasheepError::Unauthorised => None,
        }
    }
}

pub type Result<T> = result::Result<T, DatasheepError>;
