use crate::connection::{ConnOptions, Repo};
use crate::{NewUser, UpdateUser, User};
use bson;
use mongodb::error::Error;
use uuid::Uuid;

pub fn insert(repo: &Repo, options: &ConnOptions, create: NewUser) -> Result<User, Error> {
    match bson::to_bson(&create) {
        Ok(bson::Bson::Document(doc)) => {
            let result = repo.conn(options).insert_one(doc, None).await.unwrap();
            return Ok(result);
        }
        e => panic!(e),
    }
}
