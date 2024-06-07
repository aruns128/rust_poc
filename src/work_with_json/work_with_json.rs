extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::error::Error;

// Define a struct that matches the JSON structure
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

pub async fn get_person_details() -> Result<(), Box<dyn Error>> {
    // Example JSON data
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Deserialize JSON to a Rust struct
    let person: Person = serde_json::from_str(data)?;
    println!("Deserialized: {:?}", person);

    // Serialize the Rust struct back to JSON
    let json = serde_json::to_string(&person)?;
    println!("Serialized: {}", json);

    Ok(())
}
