use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Product {
    key: String,
    name: String,
    description: Option<String>,
    price_keys: Option<Vec<String>>,
}

impl Product {
    pub fn new(
        key: String,
        name: String,
        description: Option<String>,
        price_keys: Option<Vec<String>>,
    ) -> Self {
        Self {
            key,
            name,
            description,
            price_keys,
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

    pub fn price_keys(&self) -> Vec<String> {
        match &self.price_keys {
            Some(price_keys) => price_keys.clone(),
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
            price_keys: None,
        }
    }
}
