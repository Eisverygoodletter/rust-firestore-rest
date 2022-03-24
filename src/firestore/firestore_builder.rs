use super::structs::{Firestore, FirestoreBuilder};

use env_searcher;
impl FirestoreBuilder {
    pub fn new() -> FirestoreBuilder {
        FirestoreBuilder {
            db_url: "".to_string(),
        }
    }

    pub fn auth(&self, _auth_token: String) -> &Self {
        panic!("auth not implemented yet.");
    }
    pub fn from_url(&mut self, url: String) -> &Self {
        self.db_url = url;
        self
    }
    /// loads the firestore db_url from DB_URL specified as an environmental variable
    /// or in a .env file
    pub fn from_env_basic(&mut self) -> &Self {
        self.db_url = env_searcher::get_var("DB_URL".to_string()).unwrap();
        self
    }
    pub fn build(&self) -> Firestore {
        Firestore {
            db_url: self.db_url.clone(),
            reqwest_client: reqwest::Client::new(),
        }
    }
}