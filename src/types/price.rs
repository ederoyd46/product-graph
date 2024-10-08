use serde::{Deserialize, Serialize};

use crate::types::Storable;

use super::NewProduct;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Price {
    product_key: String,
    currency: String,
    country: String,
    amount: f64,
}

impl Storable for Price {
    fn db_key(&self) -> String {
        format!(
            "price:`{}:{}:{}`",
            self.product_key, self.currency, self.country
        )
    }
}

impl Price {
    pub fn new(product_key: String, currency: String, country: String, amount: f64) -> Self {
        Self {
            product_key,
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
            product_key: "UNUSABLE".to_string(),
            currency: "GBP".to_string(),
            country: "GB".to_string(),
            amount: 0.0,
        }
    }
}

impl From<NewProduct> for Vec<Price> {
    fn from(new_product: NewProduct) -> Self {
        match new_product.price {
            Some(prices) => prices
                .into_iter()
                .map(|new_price| {
                    Price::new(
                        new_product.key.clone(),
                        new_price.currency,
                        new_price.country,
                        new_price.amount,
                    )
                })
                .collect(),
            None => vec![],
        }
    }
}
