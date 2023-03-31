use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};

mod new_inventory;
mod new_price;
mod new_product;

pub use new_inventory::NewInventory;
pub use new_price::NewPrice;
pub use new_product::NewProduct;

mod view_price;
mod view_product;

use view_price::ViewPrice;
use view_product::ViewProduct;

use crate::types::ApplicationContext;

impl juniper::Context for ApplicationContext {}

use crate::services::product::mutate_product;

pub struct QueryRoot;

#[juniper::graphql_object(Context = ApplicationContext)]
impl QueryRoot {
    async fn view_product(
        &self,
        key: String,
        _context: &'a ApplicationContext,
    ) -> FieldResult<ViewProduct> {
        Ok(ViewProduct {
            key,
            name: "test".to_string(),
            description: Some("test".to_string()),
            price: Some(ViewPrice {
                amount: 1.0,
                currency_code: "EUR".to_string(),
            }),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = ApplicationContext)]
impl MutationRoot {
    async fn product<'a>(
        &self,
        context: &'a ApplicationContext,
        new_product: NewProduct,
    ) -> FieldResult<String> {
        log::info!("Mutating product: {:?}", new_product);
        mutate_product(context, new_product).await?;
        Ok("test".to_string())
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<ApplicationContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::default())
}
