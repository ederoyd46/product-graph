use serde_derive::{Deserialize, Serialize};

use crate::types::Storable;

use super::{NewProduct, Price, ProductQueryResults, QueryResult};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Product {
    key: String,
    name: String,
    description: Option<String>,
    price: Option<Vec<String>>,
}

impl Storable for Product {
    fn db_key(&self) -> String {
        format!("product:`{}`", self.key)
    }
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

impl From<ProductQueryResults> for Product {
    fn from(results: ProductQueryResults) -> Self {
        results.iter().next().unwrap().into()
    }
}

impl From<&QueryResult<Product>> for Product {
    fn from(result: &QueryResult<Product>) -> Self {
        let products = &result.result;
        let product = products.iter().next().unwrap();
        product.to_owned()
    }
}

impl From<NewProduct> for Product {
    fn from(new_product: NewProduct) -> Self {
        let product = new_product.clone();
        Self::new(
            product.key,
            product.name,
            product.description,
            match { product.price } {
                Some(_) => {
                    let prices = Vec::<Price>::from(new_product);
                    Some(prices.into_iter().map(|price| price.db_key()).collect())
                }
                None => None,
            },
        )
    }
}
