use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 5;

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempts -= 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! [attempts left: {}]", attempts),
            Ordering::Greater => println!("Too big! [attempts left: {}]", attempts),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("\n");

        if attempts == 0 {
            println!("You lost! You ran out of your attempts to guess.");
            println!("The secret number was: {}", secret_number);
            break;
        }
    }
}
