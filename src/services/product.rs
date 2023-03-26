mod mutate;
pub mod types;
pub use mutate::mutate_product;
use serde::Serialize;

use crate::types::Storable;

pub fn build_mutate_statement<T: Storable + Serialize>(o: &T) -> String {
    format!(
        "update {} content {}",
        o.db_key(),
        serde_json::to_string(&o).unwrap()
    )
}
