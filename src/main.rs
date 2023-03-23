extern crate firebase_rs as firebase;
extern crate serde;
extern crate serde_json;
extern crate tokio;

mod utils;
mod data;
mod repo;

use crate::data::models::User;

#[tokio::main]
async fn main() {
    let firebase = match firebase::Firebase::new("https://fb-rust-default-rtdb.firebaseio.com") {
        Ok(firebase) => firebase,
        Err(err) => {
            eprintln!("Cannot instantiate firebase because of an error: {}.", err);
            return;
        }
    };

    let repo = crate::repo::Repo::new(firebase, "users");

    let user = User {
        name: "John".to_owned(),
        email: "john@example.com".to_owned(),
        age: 42
    };

    if let Some(response_data) = repo.insert_user(&user).await {
        println!("inserted user response: {:?}", response_data);
    }
}
