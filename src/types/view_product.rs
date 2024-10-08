use super::{view_price::ViewPrice, Product};

pub struct ViewProduct {
    key: String,
    name: String,
    price: Option<Vec<ViewPrice>>,
    description: Option<String>,
}

#[juniper::graphql_object]
#[graphql(description = "A basic product representation")]
impl ViewProduct {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> Option<&Vec<ViewPrice>> {
        self.price.as_ref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn get_info(&self) -> String {
        format!("{}: {}", self.key, self.name)
    }
}

impl From<Product> for ViewProduct {
    fn from(product: Product) -> Self {
        Self {
            key: product.key().to_string(),
            name: product.name().to_string(),
            description: product.description().map(|s| s.to_string()),
            price: Some(
                product
                    .price()
                    .into_iter()
                    .map(|p| p.into())
                    .collect::<Vec<ViewPrice>>(),
            ),
        }
    }
}
