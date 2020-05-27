use crate::repositories::Repository;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Password(String);

impl Password {
    /// Check that a password matches `self` when hashed.
    pub fn verify(&self, clear_text_password: &str) -> Result<bool, PasswordError> {
        //Ok(bcrypt::verify(clear_text_password, &self.0)?)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SignUp {
    pub first_name: String,
    pub last_name: String,
    pub password: Password,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub _id: Uuid,
    pub profile: Profile,
}
