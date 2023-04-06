use juniper::GraphQLInputObject;

use super::{new_inventory::NewInventory, new_price::NewPrice};

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "A new product structure")]
pub struct NewProduct {
    pub name: String,
    pub key: String,
    pub price: Option<Vec<NewPrice>>,
    pub description: Option<String>,
    pub inventory: Option<Vec<NewInventory>>,
}
