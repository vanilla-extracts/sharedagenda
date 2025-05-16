use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginPost<'r> {
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginAnswer {
    pub code: i64,
    pub token: String,
    pub expiration: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogoutPost<'r> {
    pub token: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogoutAnswer {
    pub code: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserModifyPost<'r> {
    pub token: &'r str,
    pub name: &'r str,
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserModifyAnswer {
    pub code: i32,
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterPost<'r> {
    pub name: &'r str,
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterAnswer {
    pub code: i32,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserListAnswer {
    code: u16,
    users: Vec<User>,
}
