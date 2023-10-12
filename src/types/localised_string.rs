use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "A new product structure")]
pub struct LocalisedString {
    language: String,
    text: String,
}
