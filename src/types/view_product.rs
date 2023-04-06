use super::{view_price::ViewPrice, Product};

// #[derive(GraphQLObject, Debug, Clone)]

pub struct ViewProduct {
    pub key: String,
    pub name: String,
    pub price: Option<ViewPrice>,
    pub description: Option<String>,
}

#[juniper::graphql_object]
#[graphql(description = "A basic product representation")]
impl ViewProduct {
    pub fn get_info(&self) -> String {
        format!("{}: {}", self.key, self.name)
    }
}

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
