use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "A new price structure")]
pub struct NewPrice {
    pub country: String,
    pub currency: String,
    pub amount: f64,
}
