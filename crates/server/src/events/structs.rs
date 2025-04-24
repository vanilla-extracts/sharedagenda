use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::database::QueriedData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub date_start: DateTime<FixedOffset>,
    pub date_end: DateTime<FixedOffset>,
}

impl QueriedData for Event {
    fn len() -> usize {
        5_usize
    }
    fn create_from_row(row: &Row) -> Self {
        Self {
            id: row.get(0),
            name: row.get(1),
            owner: row.get(2),
            date_start: row.get(3),
            date_end: row.get(4),
        }
    }
}
