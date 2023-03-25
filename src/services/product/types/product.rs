use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Product {
    key: String,
    name: String,
    description: Option<String>,
    price: Option<Vec<String>>,
}

impl Product {
    pub fn new(
        key: String,
        name: String,
        description: Option<String>,
        price: Option<Vec<String>>,
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

    pub fn price(&self) -> Vec<String> {
        match &self.price {
            Some(price) => price.clone(),
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
