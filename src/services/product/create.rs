use log::info;

use crate::schema::NewProduct;
use crate::types::{ApplicationContext, ApplicationError, Price, Product};

pub async fn create_product(
    context: &ApplicationContext,
    new_product: NewProduct,
) -> Result<(), ApplicationError> {
    let product: Product = new_product.clone().into();
    // let prices: Vec<Price> = new_product.clone().into();

    let mut statements: Vec<String> = vec![];
    statements.push("begin transaction;".to_string());
    statements.push(format!(
        "create product:`{}` content {};",
        new_product.key,
        serde_json::to_string(&product).unwrap()
    ));

    // if new_product.price.is_some() {
    //     let prices: Vec<Price> = new_product.into();
    //     statements.push(format!(
    //         "create price set values={};",
    //         serde_json::to_string(&prices).unwrap()
    //     ));
    // }
    if new_product.price.is_some() {
        let prices: Vec<Price> = new_product.clone().into();
        for price in prices {
            statements.push(format!(
                "create price:`{}:{}` content {};",
                product.key,
                price.currency(),
                serde_json::to_string(&price).unwrap()
            ));
        }
    }

    statements.push("commit transaction;".to_string());

    // info!("{}", sql);

    let product_response = context
        .database
        .reqwest_builder(reqwest::Method::POST, "sql".to_string())
        .body(statements.concat())
        .send()
        .await?;

    info!("{:?}", product_response.text().await);
    // let product_response = context
    //     .database
    //     .reqwest_builder(
    //         reqwest::Method::POST,
    //         format!("key/product/{}", new_product.key),
    //     )
    //     .json(&product)
    //     .send()
    //     .await?;

    // info!(
    //     "Product response: {:?}",
    //     product_response.json::<Product>().await
    // );

    // let price_response = context
    //     .database
    //     .reqwest_builder(
    //         reqwest::Method::POST,
    //         format!("key/price/{}", new_product.key),
    //     )
    //     .json(&prices)
    //     .send()
    //     .await?;

    // info!("Price response: {:?}", price_response);

    Ok(())
}

impl From<NewProduct> for Product {
    fn from(new_product: NewProduct) -> Self {
        Self::new(new_product.key, new_product.name, new_product.description)
    }
}

impl From<NewProduct> for Vec<Price> {
    fn from(new_product: NewProduct) -> Self {
        match new_product.price {
            Some(prices) => prices
                .into_iter()
                .map(|new_price| Price::new(new_price.currency, new_price.amount))
                .collect(),
            None => vec![],
        }
    }
}
