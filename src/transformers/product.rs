use crate::{
    services::{
        graph::schema::NewProduct,
        product::types::{Price, Product, ProductQueryResults, QueryResult},
    },
    types::Storable,
};

impl From<ProductQueryResults> for Product {
    fn from(results: ProductQueryResults) -> Self {
        results.iter().next().unwrap().into()
    }
}

impl From<&QueryResult<Product>> for Product {
    fn from(result: &QueryResult<Product>) -> Self {
        let products = &result.result;
        let product = products.iter().next().unwrap();
        product.to_owned()
    }
}

impl From<NewProduct> for Product {
    fn from(new_product: NewProduct) -> Self {
        let product = new_product.clone();
        Self::new(
            product.key,
            product.name,
            product.description,
            match { product.price } {
                Some(_) => {
                    let prices = Vec::<Price>::from(new_product);
                    Some(prices.into_iter().map(|price| price.db_key()).collect())
                }
                None => None,
            },
        )
    }
}
