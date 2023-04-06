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
        .await?;

    let results: ProductQueryResults = product_response.json().await?;

    Ok(results.into())
}
