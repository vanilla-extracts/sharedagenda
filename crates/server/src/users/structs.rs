use chrono::{DateTime, Days, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::database::QueriedData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub owner: String,
    pub expiration_date: DateTime<Utc>,
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

impl QueriedData for Token {
    fn len() -> usize {
        3_usize
    }

    fn create_from_row(row: &Row) -> Self {
        Self {
            token: row.get(0),
            owner: row.get(1),
            expiration_date: row.get(2),
        }
    }
}

impl Token {
    pub fn new(owner: String) -> Self {
        Self {
            token: Uuid::new_v4().to_string(),
            owner,
            expiration_date: match Utc::now().checked_add_days(Days::new(1)) {
                Some(date) => date,
                None => Utc::now(),
            },
        }
    }
}
