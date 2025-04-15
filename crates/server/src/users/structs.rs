use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::database::QueriedData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

impl QueriedData for User {
    fn len() -> usize {
        4_usize
    }
    fn create_from_row(row: &Row) -> Self {
        Self {
            uuid: row.get(0),
            email: row.get(1),
            name: row.get(2),
            password: row.get(3),
        }
    }
}
