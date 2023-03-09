use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Product {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
}

impl Product {
    pub fn new(key: String, name: String, description: Option<String>) -> Self {
        Self {
            key,
            name,
            description,
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
}

impl Default for Product {
    fn default() -> Self {
        Self {
            key: "".to_string(),
            name: "".to_string(),
            description: None,
        }
    }
}
