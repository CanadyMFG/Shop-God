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
    pub updated_at: DateTime<Utc>
}
