use crate::snippets::employee_automation::json_handlers::{
    read_employees_from_file, write_employees_to_file,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub age: u8,
    pub gender: char,
    pub nationality: String,
    pub mother_tongue: String,
    pub department: String,
    pub spoken_languages: Vec<String>,
    pub programming_languages: Vec<String>,
}

impl Employee {
    pub fn employee_identity(&self) -> String {
        let gender = if self.gender == 'M' { "Male" } else { "Female" };

        format!(
            r#"
        -- {} -- 
        Age: {}
        Gender: {}
        Nationality: {}
        Mother Tongue: {}
        Department: {}
        Spoken Languages: {:?}
        Programming Languages: {:?}
        "#,
            self.name,
            self.age,
            gender,
            self.nationality,
            self.mother_tongue,
            self.department,
            self.spoken_languages,
            self.programming_languages
        )
    }

    pub fn add_programming_languages(&mut self, langs: &[&str]) {
        self.programming_languages
            .extend(langs.iter().map(|&lang| lang.to_string()));
    }

    pub fn add_spoken_languages(&mut self, langs: &[&str]) {
        self.spoken_languages
            .extend(langs.iter().map(|&lang| lang.to_string()));
    }
}

impl Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pronoun = if self.gender == 'M' { "He" } else { "She" };
        let employee_description = format!(
            "This is {} at the age {}. {} can properly develop softwares using the technologies {:?}",
            self.name, self.age, pronoun, self.programming_languages
        );
        write!(f, "{}", employee_description)
    }
}

pub fn save_employee(employee: Employee, file_path: &str) {
    let mut employees = read_employees_from_file(file_path);
    employees.push(employee);

    write_employees_to_file(&employees, file_path);
}

pub fn remove_employee(name: &str, file_path: &str) -> Option<Employee> {
    let mut employees = read_employees_from_file(file_path);
    let mut removed_employee: Option<Employee> = None;
    let mut removed = false;

    employees.retain(|employee| {
        let retain = employee.name != name;
        if !retain {
            removed = true;
            removed_employee = Some(employee.clone());
        }
        retain
    });

    if removed {
        println!("Employee with name '{}' has been removed.", name);
    } else {
        println!("No employee with name '{}' found.", name);
    }

    write_employees_to_file(&employees, file_path);

    removed_employee
}
