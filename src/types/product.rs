use serde_derive::{Deserialize, Serialize};

use super::Price;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Product {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<Vec<Price>>,
}

impl Product {
    pub fn new(
        key: String,
        name: String,
        description: Option<String>,
        price: Option<Vec<Price>>,
    ) -> Self {
        Self {
            key,
            name,
            description,
            price,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn prices(&self) -> Option<&Vec<Price>> {
        self.price.as_ref()
    }

    pub fn price_keys(&self) -> Vec<String> {
        match &self.price {
            Some(prices) => prices
                .iter()
                .map(|price| format!("{}:{}", self.key, price.currency()))
                .collect(),
            None => vec![],
        }
    }
}

impl Default for Product {
    fn default() -> Self {
        Self {
            key: "".to_string(),
            name: "".to_string(),
            description: None,
            price: None,
        }
    }
}
