use serde::export::Formatter;
use std::fmt;
use std::fmt::Debug;
use std::{error, result};

#[derive(Debug)]
pub enum SyncifyError {
    InternalServer,
    BadRequest,
    Unauthorised,
}

impl fmt::Display for SyncifyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SyncifyError::InternalServer => write!(f, "Internal Server Error"),
            SyncifyError::BadRequest => write!(f, "Bad Request"),
            SyncifyError::Unauthorised => write!(f, "Unauthorised"),
        }
    }
}

impl error::Error for SyncifyError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SyncifyError::InternalServer => None,
            SyncifyError::BadRequest => None,
            SyncifyError::Unauthorised => None,
        }
    }
}

pub type Result<T> = result::Result<T, SyncifyError>;
