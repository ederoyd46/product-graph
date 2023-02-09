use juniper::GraphQLObject;

use super::price::Price;

#[derive(GraphQLObject, Debug, Clone)]
#[graphql(description = "A basic product representation")]
pub struct Product {
    pub key: String,
    pub name: String,
    pub price: Option<Price>,
    pub description: Option<String>,
}
