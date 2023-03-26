mod context;
pub mod error;

pub use context::{ApplicationContext, ApplicationContextBuilder};
pub use error::AppError;

// Alias for a type-erased error type.
pub type ApplicationError = Box<dyn std::error::Error + Send + Sync>;

pub trait Storable {
    fn db_key(&self) -> String;
}
