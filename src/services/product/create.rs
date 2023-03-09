use log::info;

use crate::schema::NewProduct;
use crate::types::{ApplicationContext, ApplicationError, Price, Product};

pub async fn create_product(
    context: &ApplicationContext,
    new_product: NewProduct,
) -> Result<(), ApplicationError> {
    let product: Product = new_product.clone().into();
    // let prices: Vec<Price> = new_product.clone().into();

    let product_response = context
        .database
        .reqwest_builder(
            reqwest::Method::POST,
            format!("key/product/{}", new_product.key),
        )
        .json(&product)
        .send()
        .await?;

    info!("Product response: {:?}", product_response);

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
