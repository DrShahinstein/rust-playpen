use crate::snippets::employee_automation::employee::Employee;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn is_json_file_empty(file_path: &str) -> bool {
    let path = Path::new(file_path);
    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            return contents.trim().is_empty();
        }
    }
    false
}

pub fn initialize_json_file(file_path: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    file.write_all(b"[]").unwrap();
}

pub fn read_employees_from_file(file_path: &str) -> Vec<Employee> {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    let employees: Vec<Employee> = serde_json::from_str(&contents).unwrap_or_else(|_| {
        println!("Failed to deserialize employees from JSON");
        Vec::new()
    });

    employees
}

pub fn write_employees_to_file(employees: &[Employee], file_path: &str) {
    let json_data = serde_json::to_string_pretty(&employees).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to open file");

    file.write_all(json_data.as_bytes())
        .expect("Failed to write file");
}
