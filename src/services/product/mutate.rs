use log::debug;

use crate::services::product::build_mutate_statement;
use crate::types::{
    error::UnexpectedError, ApplicationContext, ApplicationError, NewProduct, Price, Product,
    Products,
};

// select *, price[where currency='GBP'] from product fetch price;

pub async fn mutate_product(
    context: &ApplicationContext,
    new_product: NewProduct,
) -> Result<Product, ApplicationError> {
    let products = Products::from(new_product.clone());
    // let product = Product::from(new_product.clone());
    let db = context.database.init_database_connection().await?;
    let mut statements: Vec<String> = vec![];

    if new_product.price.is_some() {
        let prices: Vec<Price> = new_product.clone().into();
        prices
            .iter()
            .for_each(|price| statements.push(build_mutate_statement(price)));
    }
    statements.push(build_mutate_statement(&products[0]));
    debug!("REQUEST {:?}", statements);

    let product_response = db
        .query("begin transaction")
        .query(statements.join(";"))
        .query("commit transaction")
        .await
        .map_err(|e| ApplicationError::Unexpected(UnexpectedError::new(e.to_string(), e.into())))?;

    let mut result = product_response.check().map_err(|e| {
        ApplicationError::Unexpected(UnexpectedError::new(
            "Could not parse product response".to_string(),
            e.into(),
        ))
    })?;

    let product: Option<Product> = result.take(result.num_statements() - 1).unwrap();

    debug!("RESPONSE {:?}", result);

    Ok(product.unwrap())
}
