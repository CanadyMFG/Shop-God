use crate::{NewUser, UpdateUser};
use crate::Repo;
use anyhow::Error as OpaqueError;
use uuid::Uuid;
use domain;

pub fn to_db_error(e: Error) -> domain::DatabaseError -> {
    domain::DatabaseError::from(OpaqueError::from(e))
}

#[derive(Clone)]
pub struct Repository(pub Repo);

impl domain::repositories::Repository for Repository {}
