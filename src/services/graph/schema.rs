use crate::types::{ApplicationContext, NewProduct, ViewProduct};
use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
impl juniper::Context for ApplicationContext {}

use crate::services::product::{mutate_product, query_product, query_products};

pub struct QueryRoot;

#[graphql_object(Context = ApplicationContext)]
impl QueryRoot {
    async fn product(&self, key: String, context: &ApplicationContext) -> FieldResult<ViewProduct> {
        let found_product = query_product(context, &key).await?;
        Ok(found_product.into())
    }
    async fn products(&self, context: &ApplicationContext) -> FieldResult<Vec<ViewProduct>> {
        let found_product = query_products(context).await?;
        let mapped_products = found_product.into_iter().map(ViewProduct::from).collect();
        Ok(mapped_products)
    }
}

pub struct MutationRoot;

#[graphql_object(Context = ApplicationContext)]
impl MutationRoot {
    async fn product(
        &self,
        context: &ApplicationContext,
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
