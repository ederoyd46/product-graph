use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Price {
    currency: String,
    country: String,
    amount: f64,
}

impl Price {
    pub fn new(currency: String, country: String, amount: f64) -> Self {
        Self {
            currency,
            country,
            amount,
        }
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}

impl Default for Price {
    fn default() -> Self {
        Self {
            currency: "GBP".to_string(),
            country: "GB".to_string(),
            amount: 0.0,
        }
    }
}
