use failure::Error;
use crate::presentation::web::error::{ValidationError, NotFoundError};
use std::error::Error as StdError;
use std::convert::From;
use std::fmt;

#[derive(Debug)]
pub enum WebError {
    Failure(Error),
    Validation(ValidationError),
    NotFound(NotFoundError),
}

impl From<Error> for WebError {
    fn from(error: Error) -> Self {
        return WebError::Failure(error);
    }
}

impl From<ValidationError> for WebError {
    fn from(error: ValidationError) -> Self {
        return WebError::Validation(error);
    }
}

impl From<NotFoundError> for WebError {
    fn from(error: NotFoundError) -> Self {
        return WebError::NotFound(error);
    }
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &WebError::Failure(ref error) => {
                write!(f, "{}", error)
            },
            &WebError::Validation(ref error) => {
                write!(f, "{}", error.get_reason())
            },
            &WebError::NotFound(ref error) => {
                write!(f, "{}", error.get_reason())
            },
        }
    }
}

impl StdError for WebError {}