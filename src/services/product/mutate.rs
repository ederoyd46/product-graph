use log::debug;

use crate::schema::{NewPrice, NewProduct};
use crate::services::product::types::{Price, Product};
use crate::types::{ApplicationContext, ApplicationError};

// select *, price[where currency='GBP'] from product fetch price;

pub async fn mutate_product(
    context: &ApplicationContext,
    new_product: NewProduct,
) -> Result<(), ApplicationError> {
    let product = Product::from(new_product.clone());

    let mut statements: Vec<String> = vec![];
    statements.push("begin transaction".to_string());

    if new_product.price.is_some() {
        let prices: Vec<Price> = new_product.clone().into();
        for price in prices {
            statements.push(format!(
                "update price:`{}:{}` content {}",
                product.key(),
                price.currency(),
                serde_json::to_string(&price).unwrap()
            ));
        }
    }

    statements.push(format!(
        "update product:`{}` content {}",
        new_product.key,
        serde_json::to_string(&product).unwrap(),
    ));

    statements.push("commit transaction;".to_string());

    debug!("REQUEST {:?}", statements);

    let product_response = context
        .database
        .reqwest_builder(reqwest::Method::POST, "sql")
        .body(statements.join(";"))
        .send()
        .await?;

    debug!("RESPONSE: {:?}", product_response.text().await);
    Ok(())
}

impl From<NewProduct> for Product {
    fn from(new_product: NewProduct) -> Self {
        Self::new(
            new_product.key.clone(),
            new_product.name,
            new_product.description,
            match { new_product.price } {
                Some(prices) => Some(
                    prices
                        .into_iter()
                        .map(|new_price| {
                            format!("price:`{}:{}`", new_product.key, new_price.currency)
                        })
                        .collect(),
                ),
                None => None,
            },
        )
    }
}

impl From<NewProduct> for Vec<Price> {
    fn from(new_product: NewProduct) -> Self {
        match new_product.price {
            Some(prices) => prices
                .into_iter()
                .map(|new_price| {
                    Price::new(new_price.currency, new_price.country, new_price.amount)
                })
                .collect(),
            None => vec![],
        }
    }
}
impl From<NewPrice> for Price {
    fn from(new_price: NewPrice) -> Self {
        Self::new(new_price.currency, new_price.country, new_price.amount)
    }
}
