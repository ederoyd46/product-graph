use log::debug;

use crate::services::product::build_mutate_statement;
use crate::types::{ApplicationContext, ApplicationError, NewProduct, Price, Product};

// select *, price[where currency='GBP'] from product fetch price;

pub async fn mutate_product(
    context: &ApplicationContext,
    new_product: NewProduct,
) -> Result<Product, ApplicationError> {
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
        .await
        .map_err(|e| ApplicationError::new(e.to_string()))?;

    debug!("RESPONSE {:?}", &product_response.text().await.unwrap());

    // let results: ProductQueryResults = product_response
    //     .text()
    //     .await
    //     .map_err(|e| ApplicationError::new(e.to_string()))?;

    Ok(Product::from(new_product))
}
