use crate::types::{ApplicationContext, ApplicationError};

use super::types::Product;

pub async fn query_product(
    context: &ApplicationContext,
    key: String,
) -> Result<Product, ApplicationError> {
    let product_response = context
        .database
        .reqwest_builder(reqwest::Method::GET, "product")
        .query(&[("key", key)])
        .send()
        .await?;

    let product: Product = product_response.json().await?;

    Ok(product)
}
