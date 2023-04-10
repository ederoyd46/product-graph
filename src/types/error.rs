use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ApplicationError {
    pub kind: String,
    pub message: String,
}

impl Error for ApplicationError {}

impl ApplicationError {
    pub fn new(message: String) -> Self {
        Self {
            kind: "ApplicationError".to_string(),
            message,
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{ kind: {}, message: {} }}",
            self.kind, self.message
        )
    }
}
