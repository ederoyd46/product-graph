mod price;
mod product;
mod query_result;

pub use price::Price;
pub use product::Product;
pub use query_result::{ProductQueryResults, QueryResult, QueryResults};

mod new_inventory;
mod new_price;
mod new_product;

pub use new_inventory::NewInventory;
pub use new_price::NewPrice;
pub use new_product::NewProduct;

mod view_price;
mod view_product;

pub use view_price::ViewPrice;
pub use view_product::ViewProduct;

mod context;
pub mod error;

pub use context::{ApplicationContext, ApplicationContextBuilder};
pub use error::ApplicationError;

// Alias for a type-erased error type.
// pub type ApplicationError = Box<dyn std::error::Error + Send + Sync>;

pub trait Storable {
    fn db_key(&self) -> String;
}
