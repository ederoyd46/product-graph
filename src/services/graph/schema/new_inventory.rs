use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject, Debug, Clone)]
#[graphql(description = "A new inventory structure")]
pub struct NewInventory {
    pub warehouse: Option<String>,
    pub country: Option<String>,
    pub quantity: i32,
}
