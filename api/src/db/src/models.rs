use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    pub _id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, AsChangeset, Default, Clone)]
pub struct UpdateUser<'a> {
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, AsChangeset, Default, Clone)]
pub struct NewUser<'a> {
    pub _id: Uuid,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password: &'a str,
}
