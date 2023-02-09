use juniper::GraphQLObject;

#[derive(GraphQLObject, Debug, Clone)]
#[graphql(description = "A price structure")]
pub struct Price {
    pub currency_code: String,
    pub amount: f64,
}
