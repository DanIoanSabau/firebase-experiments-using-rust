pub mod json {
    pub mod deserialize {
        pub fn to<'a, T>(serialized_data: &'a str) -> T where T: serde::Deserialize<'a> {
            println!("deserializing json:\n{}", serialized_data);
            serde_json::from_str(serialized_data).unwrap()
        }
    }
}