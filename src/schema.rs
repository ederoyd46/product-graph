use juniper::{EmptyMutation, FieldResult};
use juniper::{EmptySubscription, RootNode};

mod price;
mod product;
use product::Product;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    async fn view_product(key: String) -> FieldResult<Product> {
        Ok(Product {
            key,
            name: "test".to_string(),
            description: Some("test".to_string()),
            price: Some(price::Price {
                amount: 1.0,
                currency_code: "EUR".to_string(),
            }),
        })
    }
}

// pub struct MutationRoot;

// #[juniper::graphql_object]
// impl MutationRoot {
// async fn create_product(new_product: NewProduct) -> FieldResult<Product> {
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

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot {},
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
}
