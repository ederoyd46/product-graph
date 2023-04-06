use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};

use crate::types::{ApplicationContext, NewProduct, ViewProduct};

impl juniper::Context for ApplicationContext {}

use crate::services::product::{mutate_product, query_product};

pub struct QueryRoot;

#[juniper::graphql_object(Context = ApplicationContext)]
impl QueryRoot {
    async fn product(
        &self,
        key: String,
        context: &'a ApplicationContext,
    ) -> FieldResult<ViewProduct> {
        let result = query_product(context, &key).await?;
        Ok(result.into())
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
