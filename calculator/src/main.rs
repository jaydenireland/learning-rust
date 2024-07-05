use std::io::{self, Write};
use std::process::exit;

fn main() {

    loop {
        print!("Enter first number: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let first_number: i32 = input.trim().parse().expect("Please type a valid number!");

        print!("Enter second number: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let second_number: i32 = input.trim().parse().expect("Please enter a valid number");

        print!("Enter operation: ");
        io::stdout().flush().unwrap();
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line");
        let operation = operation.trim();

        match operation {
            "+" => {
                let result: i32 = first_number + second_number;
                println!("{} + {} = {}", first_number, second_number, result);
                continue;
            }
            "-" => {
                let result: i32 = first_number - second_number;
                println!("{} - {} = {}", first_number, second_number, result);
                continue;

            }
            "/" => {
                let result: i32 = first_number / second_number;
                println!("{} / {} = {}", first_number, second_number, result);
                continue;
            }
            "*"|"x" => {
                let result: i32 = first_number * second_number;
                println!("{} * {} = {}", first_number, second_number, result);
                continue;
            }
            _ => {
                println!("Unsupported operation!");
            }
        }
    }


}
