mod context;
pub mod error;
mod price;
mod product;

pub use context::{ApplicationContext, ApplicationContextBuilder};
pub use error::AppError;
pub use price::Price;
pub use product::Product;

// Alias for a type-erased error type.
pub type ApplicationError = Box<dyn std::error::Error + Send + Sync>;
