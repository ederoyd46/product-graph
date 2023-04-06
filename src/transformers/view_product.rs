use crate::services::{graph::schema::ViewProduct, product::types::Product};

impl From<Product> for ViewProduct {
    fn from(product: Product) -> Self {
        Self {
            key: product.key().to_string(),
            name: product.name().to_string(),
            description: None,
            price: None,
        }
    }
}
