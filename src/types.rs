mod localised_string;
mod price;
mod product;

pub use price::Price;
pub use product::{Product, Products};

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

pub trait Storable {
    fn db_key(&self) -> String;
}
