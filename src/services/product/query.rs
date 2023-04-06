use crate::types::{ApplicationContext, ApplicationError, Product, ProductQueryResults};

pub async fn query_product(
    context: &ApplicationContext,
    key: &str,
) -> Result<Product, ApplicationError> {
    let product_response = context
        .database
        .reqwest_builder(
            reqwest::Method::GET,
            format!("key/product/{}", key).as_str(),
        )
        .send()
        .await
        .map_err(|e| ApplicationError::new(e.to_string()))?;

    let results: ProductQueryResults = product_response
        .json()
        .await
        .map_err(|e| ApplicationError::new(e.to_string()))?;

    Ok(results.into())
}
