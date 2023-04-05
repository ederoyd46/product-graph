use crate::types::{ApplicationContext, ApplicationError};

use super::types::{Product, ProductQueryResults, QueryResult};

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
