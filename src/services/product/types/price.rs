use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Price {
    currency: String,
    amount: f64,
}

impl Price {
    pub fn new(currency: String, amount: f64) -> Self {
        Self { currency, amount }
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }
}

impl Default for Price {
    fn default() -> Self {
        Self {
            currency: "GBP".to_string(),
            amount: 0.0,
        }
    }
}
