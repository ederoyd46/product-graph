use log::debug;

use crate::types::{error::UnexpectedError, ApplicationContext, ApplicationError, Product};

pub async fn query_product(
    context: &ApplicationContext,
    key: &str,
) -> Result<Product, ApplicationError> {
    let db = context.database.init_database_connection().await?;

    let result: Option<Product> = db.select(("product", key)).await.map_err(|e| {
        ApplicationError::Unexpected(UnexpectedError::new(
            "Could not select from DB".into(),
            e.into(),
        ))
    })?;

    debug!("Product Found {:?}", result);

    match result {
        Some(product) => Ok(product),
        None => Err(ApplicationError::NotFound),
    }
}

pub async fn query_products(
    context: &ApplicationContext,
) -> Result<Vec<Product>, ApplicationError> {
    let db = context.database.init_database_connection().await?;

    let results: Vec<Product> = db.select("product").await.map_err(|e| {
        ApplicationError::Unexpected(UnexpectedError::new(
            "Could not select from DB".into(),
            e.into(),
        ))
    })?;

    debug!("Records retrieved {:?}", results.len());
    Ok(results)
}
