use std::collections::HashMap;

use crate::data::models::User;

pub struct Repo {
    firebase: firebase::Firebase
}

impl Repo {
    pub fn new(firebase: firebase::Firebase, path: &str) -> Self {
        Repo { firebase: firebase.at(path) }
    }
}

impl Repo {
    pub async fn insert_user(& self, user: &User) -> Option<String> {
        match self.firebase.set::<User>(&user).await {
            Ok(response) => Some(response.data),
            Err(error) => {
                println!("Error encountered while insert_user called: {:?}.", error);
                None
            }
        }
    }

    pub async fn get_all(&self) -> HashMap<String, User> {
        match self.firebase.get::<HashMap<String, User>>().await {
            Ok(users) => users,
            Err(error) => {
                println!("Error encountered while get_all called: {:?}.", error);
                HashMap::new()
            }
        }
    }
}
