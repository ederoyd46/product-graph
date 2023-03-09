use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub kind: String,
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AppError {{ kind: {}, message: {} }}",
            self.kind, self.message
        )
    }
}
