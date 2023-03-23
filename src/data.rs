pub mod models {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct User {
        pub name: String,
        pub email: String,
        pub age: u32
    }
}

pub mod dto {}