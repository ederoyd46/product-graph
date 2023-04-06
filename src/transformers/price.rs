use crate::services::{graph::schema::NewProduct, product::types::Price};

impl From<NewProduct> for Vec<Price> {
    fn from(new_product: NewProduct) -> Self {
        match new_product.price {
            Some(prices) => prices
                .into_iter()
                .map(|new_price| {
                    Price::new(
                        new_product.key.clone(),
                        new_price.currency,
                        new_price.country,
                        new_price.amount,
                    )
                })
                .collect(),
            None => vec![],
        }
    }
}
