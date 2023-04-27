use juniper::GraphQLObject;

use super::Price;

#[derive(GraphQLObject, Debug, Clone)]
#[graphql(description = "A price structure")]
pub struct ViewPrice {
    pub currency_code: String,
    pub amount: f64,
}

impl From<Price> for ViewPrice {
    fn from(price: Price) -> Self {
        ViewPrice {
            currency_code: price.currency().to_string(),
            amount: price.amount(),
        }
    }
}
