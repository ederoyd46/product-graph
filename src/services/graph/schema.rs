use crate::types::{ApplicationContext, NewProduct, ViewProduct};
use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
impl juniper::Context for ApplicationContext {}

use crate::services::product::{mutate_product, query_product};

pub struct QueryRoot;

#[graphql_object(Context = ApplicationContext)]
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

#[graphql_object(Context = ApplicationContext)]
impl MutationRoot {
    async fn product<'a>(
        &self,
        context: &'a ApplicationContext,
        new_product: NewProduct,
    ) -> FieldResult<ViewProduct> {
        log::info!("Mutating product: {:?}", new_product);
        let result = mutate_product(context, new_product).await?;
        Ok(result.into())
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<ApplicationContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::default())
}
