use domain;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserResponse {
    pub user: User,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub token: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}
