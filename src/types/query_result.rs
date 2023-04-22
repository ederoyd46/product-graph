use serde::{Deserialize, Serialize};

use super::Product;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryResult<T> {
    pub time: String,
    pub status: String,
    pub result: Vec<T>,
}

impl<T> QueryResult<T> {
    pub fn has_results(&self) -> bool {
        !self.result.is_empty()
    }
}

pub type QueryResults<T> = Vec<QueryResult<T>>;

pub type ProductQueryResults = QueryResults<Product>;
