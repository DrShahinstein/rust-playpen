use crate::snippets::employee_automation::employee::{remove_employee, save_employee, Employee};
use crate::snippets::employee_automation::json_handlers::{
    initialize_json_file, is_json_file_empty, read_employees_from_file,
};
use std::io;

const FILE_PATH: &str = "employees.json";
const MENU: &str = r#"
- - - - - - - - - - - - - - - - 
0- Show this message
1- Add Employees
2- Remove Employees
3- Display Available Employees 
4- Display Fired Employees
5- Exit
- - - - - - - - - - - - - - - - 
"#;

pub fn run() {
    if is_json_file_empty(FILE_PATH) {
        initialize_json_file(FILE_PATH)
    }

    println!("{}", MENU);

    loop {
        println!("\nOperation: ");
        let operation = read_input("> ");

        if operation == "0" {
            println!("{}", MENU);
        }

        if operation == "1" {
            let name = read_input("Enter name: ");
            let age = read_input("Enter age: ");
            let gender = read_input("Enter gender: ");
            let nationality = read_input("Enter nationality: ");
            let mother_tongue = read_input("Enter mother tongue: ");
            let department = read_input("Enter department: ");
            let spoken_languages = read_input("Enter spoken languages: ");
            let programming_languages = read_input("Enter programming languages: ");

            let employee = Employee {
                name,
                age: age.parse().unwrap(),
                gender: gender.chars().next().unwrap(),
                nationality,
                mother_tongue,
                department,
                spoken_languages: vectorize_languages(spoken_languages),
                programming_languages: vectorize_languages(programming_languages),
            };

            save_employee(employee, FILE_PATH);
        }

        if operation == "2" {
            let removed_employee = read_input("\nEnter the employee to be removed: ");
            remove_employee(&removed_employee, FILE_PATH);
        }

        if operation == "3" {
            println!("\n");
            let employees = read_employees_from_file(FILE_PATH);
            for (i, employee) in employees.iter().enumerate() {
                let employee_index = i + 1;
                println!("{}. {}", employee_index, employee.name);
            }

            println!("Enter the employee index to view details: ");
            let input = read_input("> ");
            let selected_index = input.parse::<usize>();

            match selected_index {
                Ok(index) => {
                    if index > 0 && index <= employees.len() {
                        let selected_employee = &employees[index - 1];
                        println!("{}", selected_employee.employee_identity());
                    } else {
                        println!("Invalid employee index.");
                    }
                }
                Err(_) => {
                    println!("Invalid input. Please enter a valid employee index.");
                }
            }
        }

        // Handle other operations (remove employees, display employees, etc.)

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
