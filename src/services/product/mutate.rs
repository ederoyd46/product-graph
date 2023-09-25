use log::debug;
use serde_json::Value;

use crate::services::product::{build_mutate_statement, query_product};
use crate::types::{
    error::UnexpectedError, ApplicationContext, ApplicationError, NewProduct, Price, Product,
    QueryResults,
};

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

    let response = context
        .database_rest
        .reqwest_builder(reqwest::Method::POST, "sql")
        .body(statements.join(";"))
        .send()
        .await
        .map_err(|e| ApplicationError::Unexpected(UnexpectedError::new(e.to_string(), e.into())))?;

    let product_response: QueryResults<Value> = response
        .json()
        .await
        .map_err(|e| ApplicationError::Unexpected(UnexpectedError::new(e.to_string(), e.into())))?;

    debug!("RESPONSE {:?}", &product_response);

    if !is_successful(product_response) {
        return Err(ApplicationError::InternalError(
            "Failed to mutate product".to_string(),
        ));
    }

    query_product(context, &new_product.key).await
}

fn is_successful(product_response: QueryResults<Value>) -> bool {
    let success = product_response
        .into_iter()
        .all(|result| result.status == "OK");

    debug!("SUCCESS {:?}", success);
    success
}
