use crate::{SignUp, SignUpError, User};
use uuid::Uuid;

pub trait Repository {
    fn sign_up(&self, sign_up: SignUp) -> Result<User, SignUpError>;
}
