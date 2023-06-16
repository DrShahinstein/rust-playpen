use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

pub fn run() {
    const FILE_PATH: &str = "./index.json";
    let person = Person {
        name: "John".to_string(),
        age: 30,
        city: "New York".to_string(),
    };

    // Write data to JSON file
    let json_data = serde_json::to_string(&person).unwrap();

    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();

    // Read data from JSON file
    let mut file = File::open(FILE_PATH).unwrap();
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).unwrap();

    let person: Person = serde_json::from_str(&json_data).unwrap();

    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("City: {}", person.city);
}
