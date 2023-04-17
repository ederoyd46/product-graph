use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ApplicationError {
    NotFound,
    InternalError(String),
    BadRequest(String),
    Unexpected(UnexpectedError),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::NotFound => write!(f, "NotFound"),
            ApplicationError::InternalError(message) => write!(f, "InternalError: {}", message),
            ApplicationError::BadRequest(message) => write!(f, "BadRequest: {}", message),
            ApplicationError::Unexpected(error) => write!(f, "UnexpectedError: {}", error),
        }
    }
}

#[derive(Debug)]
pub struct UnexpectedError {
    pub message: String,
    pub error: Box<dyn Error>,
}

impl Error for UnexpectedError {}

impl UnexpectedError {
    pub fn new(message: String, error: Box<dyn Error>) -> Self {
        Self { message, error }
    }
}

impl fmt::Display for UnexpectedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UnexpectedError {{ message: {}, error: {:?} }}",
            self.message, self.error
        )
    }
}
