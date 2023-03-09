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

use crate::services::product::create_product;

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
    async fn create_product<'a>(
        &self,
        context: &'a ApplicationContext,
        new_product: NewProduct,
    ) -> FieldResult<String> {
        log::info!("Creating product: {:?}", new_product);
        create_product(context, new_product).await?;
        Ok("test".to_string())
    }
}
//     let store_product_url =
//         env::var("STORE_PRODUCT_URL").expect("STORE_PRODUCT_URL must be set");

//     let product = Product::from(new_product);
//     let product_model: ProductModel = product.try_into().expect("count not be converted");

//     let response = reqwest::Client::new()
//         .post(store_product_url)
//         .json(&product_model)
//         .send()
//         .await;

//     match response {
//         Ok(response) => Ok(Product::from(
//             response
//                 .json::<ProductModel>()
//                 .await
//                 .expect("Failed to parse response"),
//         )),
//         Err(error) => Err(juniper::FieldError::new(
//             "Error retrieving product",
//             juniper::Value::scalar(error.to_string()),
//         )),
//     }
// }
// }

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<ApplicationContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::default())
}
