use serde::{Deserialize, Serialize};

use super::Product;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryResult<T> {
    pub time: String,
    pub status: String,
    pub result: Vec<T>,
}

pub type QueryResults<T> = Vec<QueryResult<T>>;

pub type ProductQueryResults = QueryResults<Product>;