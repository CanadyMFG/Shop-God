//#![deny(warnings)]
//use mongodb::{db::ThreadedDatabase, Client, ThreadedClient};
use mongodb::{options::ClientOptions, Client};
use std::env;
use warp::{http::StatusCode, Filter};

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "users=info");
    }
    pretty_env_logger::init();

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("mydb");

    let api = filters::users(db);

    // View access logs by setting `RUST_LOG=users`.
    let routes = api.with(warp::log("users"));
    // Start up the server...
    warp::serve(routes).run(([192, 168, 1, 42], 3030)).await;
}

mod filters {
    use super::handlers;
    use super::models::{Db, User};
    use warp::Filter;

    /// The 4 TODOs filters combined.
    pub fn users(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        users_create(db.clone())
    }

    /// POST /users with JSON body
    pub fn users_create(
        db: Db,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("users")
            .and(warp::post())
            .and(json_body())
            .and(with_db(db))
            .and_then(handlers::create_user)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {
    use super::models::{Db, User};
    use bson;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn create_user(create: User, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("create_user: {:?}", create);
        let COLLECTION = "Users";
        let collection = db.collection(COLLECTION);
        //create user

        match bson::to_bson(&create) {
            Ok(bson::Bson::Document(doc)) => {
                collection.insert_one(doc, None).await.unwrap();
            }
            e => panic!(e),
        }

        Ok(StatusCode::CREATED)
    }
}

mod models {
    use serde_derive::{Deserialize, Serialize};
    pub type Db = mongodb::Database;

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct User {
        pub first_name: String,
        pub last_name: String,
        pub password: String,
    }
}
