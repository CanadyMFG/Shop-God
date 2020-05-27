use mongodb::{options::ClientOptions, Client, Collection}

pub struct ConnOptions {
    pub db_name: String,
    pub collection: String
}

pub struct Repo {
    client: Client,
}

impl Repo {
    // It's ok to unwrap() since we will only call this once and
    // it's required for the app to startup
    pub fn new(database_url: &str) -> Self {
        let client_options = ClientOptions::parse(database_url)
            .await
            .unwrap();
        let client = Client::with_options(client_options).unwrap();
        Repo { client }
    }

    pub fn conn(&self, options: ConnOptions) -> Collection {
       self.client
           .database(options.db_name)
           .collection(options.collection)
    }
}
