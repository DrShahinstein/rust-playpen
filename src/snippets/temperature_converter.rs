use std::io::{self, Write};

pub fn run() {
    println!("Temperature Converter");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim().parse::<u32>() {
        Ok(1) => {
            let f_to_c_result = fahrenheit_to_celsius(get_temperature_input("Fahrenheit"));
            println!("F to C: {:.2} °C", f_to_c_result);
        }
        Ok(2) => {
            let c_to_f_result = celsius_to_fahrenheit(get_temperature_input("Celsius"));
            println!("C to F: {:.2} °F", c_to_f_result);
        }
        _ => println!("Invalid choice!"),
    }
}

fn get_temperature_input(temperature_type: &str) -> Temperature {
    print!("\nEnter {}: ", temperature_type);
    io::stdout().flush().unwrap();

    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).unwrap();

    let temperature: f32 = temperature.trim().parse().unwrap();

    match temperature_type {
        "Fahrenheit" => Temperature::Fahrenheit(temperature),
        "Celsius" => Temperature::Celsius(temperature),
        _ => panic!("Invalid temperature type!"),
    }
}

fn fahrenheit_to_celsius(temperature: Temperature) -> f32 {
    if let Temperature::Fahrenheit(f) = temperature {
        (5.0 * (f - 32.0)) / 9.0
    } else {
        panic!("Please enter a fahrenheit type of input.")
    }
}

fn celsius_to_fahrenheit(temperature: Temperature) -> f32 {
    if let Temperature::Celsius(c) = temperature {
        (c * 9.0 / 5.0) + 32.0
    } else {
        panic!("Please enter a celsius type of input.")
    }
}

enum Temperature {
    Celsius(f32),
    Fahrenheit(f32),
}
