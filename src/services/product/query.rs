use crate::{
    services::graph::schema::ViewProduct,
    types::{ApplicationContext, ApplicationError},
};

use super::types::{Product, ProductQueryResults, QueryResult};

pub async fn query_product(
    context: &ApplicationContext,
    key: &str,
) -> Result<ViewProduct, ApplicationError> {
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

impl From<&Product> for ViewProduct {
    fn from(product: &Product) -> Self {
        Self {
            key: product.key().to_string(),
            name: product.name().to_string(),
            description: None,
            price: None,
        }
    }
}

impl From<ProductQueryResults> for ViewProduct {
    fn from(results: ProductQueryResults) -> Self {
        results.iter().next().unwrap().into()
    }
}

impl From<&QueryResult<Product>> for ViewProduct {
    fn from(result: &QueryResult<Product>) -> Self {
        let products = &result.result;
        let product = products.iter().next().unwrap();
        return product.into();
    }
}
