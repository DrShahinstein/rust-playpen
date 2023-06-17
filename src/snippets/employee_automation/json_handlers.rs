use crate::snippets::employee_automation::employee::Employee;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

fn is_json_array(contents: &str) -> bool {
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(contents) {
        parsed.is_array()
    } else {
        false
    }
}

pub fn initialize_json(files: &[&str]) {
    for file_path in files {
        let path = Path::new(file_path);

        if path.exists() {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(file_path)
                .expect("Failed to open file");

            let mut contents = String::new();

            if file.read_to_string(&mut contents).is_ok() {
                if contents.trim().is_empty() || !is_json_array(&contents) {
                    // File is empty or doesn't have an array inside, so initialize with an empty array
                    file.seek(SeekFrom::Start(0)).expect("Failed to seek file");
                    file.set_len(0).expect("Failed to truncate file");
                    file.write_all(b"[]").expect("Failed to write to file");
                } else {
                    // Reset file position to the start
                    file.seek(SeekFrom::Start(0)).expect("Failed to seek file");
                }
            }
        } else {
            // File doesn't exist, create it and initialize with an empty array
            let mut file = File::create(file_path).expect("Failed to create file");
            file.write_all(b"[]").expect("Failed to write to file");
        }
    }
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
