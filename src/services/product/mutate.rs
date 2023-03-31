use log::debug;

use crate::services::graph::schema::NewProduct;
use crate::services::product::build_mutate_statement;
use crate::services::product::types::{Price, Product};
use crate::types::{ApplicationContext, ApplicationError, Storable};

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
            statements.push(build_mutate_statement(&price));
        }
    }

    statements.push(build_mutate_statement(&product));
    statements.push("commit transaction".to_string());

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
// impl From<NewPrice> for Price {
//     fn from(new_price: NewPrice) -> Self {
//         Self::new(new_price.currency, new_price.country, new_price.amount)
//     }
// }
