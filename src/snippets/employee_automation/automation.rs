use crate::snippets::employee_automation::employee::{remove_employee, save_employee, Employee};
use crate::snippets::employee_automation::json_handlers::{
    initialize_json, read_employees_from_file,
};
use std::io;

const EMPLOYEES_JSON: &str = "employees.json";
const REMOVED_EMPLOYEES_JSON: &str = "removed.json";
const MENU: &str = r#"
- - - - - - - - - - - - - - - - 
0- Show this message
1- Add Employees
2- Remove Employees
3- Display Available Employees 
4- Display Removed Employees
5- Exit
- - - - - - - - - - - - - - - - 
"#;

pub fn run() {
    initialize_json(&[EMPLOYEES_JSON, REMOVED_EMPLOYEES_JSON]);

    println!("{}", MENU);

    loop {
        println!("\nOperation: ");
        let operation = read_input("> ");

        if operation == "0" {
            println!("{}", MENU);
        }

        if operation == "1" {
            let name = read_input("Enter name: ");
            let age = read_input("Enter age: ")
                .parse()
                .expect("Age must be a number");
            let gender = read_input("Enter gender: ")
                .chars()
                .next()
                .expect("Gender must be an only char");
            let nationality = read_input("Enter nationality: ");
            let mother_tongue = read_input("Enter mother tongue: ");
            let department = read_input("Enter department: ");
            let spoken_languages = read_input("Enter spoken languages: ");
            let programming_languages = read_input("Enter programming languages: ");

            let employee = Employee {
                name,
                age,
                gender,
                nationality,
                mother_tongue,
                department,
                spoken_languages: vectorize_languages(spoken_languages),
                programming_languages: vectorize_languages(programming_languages),
            };

            let removed_employees = read_employees_from_file(REMOVED_EMPLOYEES_JSON);
            let employee_removed_before = removed_employees
                .iter()
                .find(|removed| removed.name == employee.name);

            if let Some(emp) = employee_removed_before {
                remove_employee(&emp.name, REMOVED_EMPLOYEES_JSON);
            }

            save_employee(employee, EMPLOYEES_JSON);
            println!("Employee added successfully!");
        }

        if operation == "2" {
            let removed_employee = read_input("\nEnter the employee to be removed: ");
            match remove_employee(&removed_employee, EMPLOYEES_JSON) {
                Some(employee) => {
                    save_employee(employee.clone(), REMOVED_EMPLOYEES_JSON);
                }
                None => {
                    println!("No employee found with the name {}", removed_employee);
                }
            }
        }

        if operation == "3" {
            view_employees(EMPLOYEES_JSON, "-- Available Employees --");
        }

        if operation == "4" {
            view_employees(REMOVED_EMPLOYEES_JSON, "-- Removed Employees --");
        }

        if operation == "5" {
            println!("\nExited.");
            break;
        }
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

fn vectorize_languages(langs_input: String) -> Vec<String> {
    langs_input
        .split(',')
        .map(|language| language.trim().to_string())
        .collect()
}

fn view_employees(file_path: &str, label: &str) {
    println!("\n{}", label);
    let employees = read_employees_from_file(file_path);

    if employees.is_empty() {
        println!("No employee found to view.");
    } else {
        for (i, employee) in employees.iter().enumerate() {
            let employee_index = i + 1;
            println!("{}. {}", employee_index, employee.name);
        }
    }

    let input = read_input("\nEnter the employee index to view details: ");
    let selected_index = input.parse::<usize>();

    match selected_index {
        Ok(index) => {
            if index > 0 && index <= employees.len() {
                let selected_employee = &employees[index - 1];
                println!("{}", selected_employee.employee_identity());
            } else {
                println!("No employee found with the index {}", index);
            }
        }
        Err(_) => {
            println!("Please enter a valid index number.");
        }
    }
}
