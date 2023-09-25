use log::info;

use crate::types::{
    error::UnexpectedError, ApplicationContext, ApplicationError, Product, ProductQueryResults,
};

pub async fn query_product(
    context: &ApplicationContext,
    key: &str,
) -> Result<Product, ApplicationError> {
    let product_response = context
        .database_rest
        .reqwest_builder(
            reqwest::Method::GET,
            format!("key/product/{}", key).as_str(),
        )
        .send()
        .await
        .map_err(|e| {
            ApplicationError::Unexpected(UnexpectedError::new(
                "Could not GET from DB".into(),
                e.into(),
            ))
        })?;

    let results = product_response
        .json::<ProductQueryResults>()
        .await
        .map_err(|e| {
            ApplicationError::Unexpected(UnexpectedError::new(
                "Unable to parse product query results".to_string(),
                e.into(),
            ))
        })?;

    if results.is_empty() {
        return Err(ApplicationError::NotFound);
    }

    let results_sdk: Vec<Product> = context
        .database_sdk
        .init_database_connection()
        .await
        .unwrap()
        .select("product")
        .await
        .unwrap();

    info!("SDK {:?}", results_sdk);

    Ok(results.into())
}
