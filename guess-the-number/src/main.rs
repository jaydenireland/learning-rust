// Import the external crate 'rand' which provides random number generation functionality
extern crate rand;
// Import the 'Rng' trait from the 'rand' crate, which defines methods for random number generation
use rand::Rng;
// Import the 'io' module from the standard library, which provides input/output functionality
use std::io;

// Define the main function, which is the entry point of the program
fn main() {
    // Create a random number generator using the thread-local random number generator
    let mut rng = rand::thread_rng();
    // Generate a random number in the range 1 to 5 and assign it to 'random_number'
    let random_number: u32 = rng.gen_range(1..5);

    // Create a mutable String to store user input
    let mut input = String::new();
    // Start an infinite loop
    loop {
        // Clear the contents of the 'input' String
        input.clear();
        // Print a prompt asking the user to enter their guess
        println!("Please enter your guess");
        // Read a line of input from the standard input (keyboard) and store it in 'input'
        io::stdin().read_line(&mut input).expect("Failed to read line");
        // Trim whitespace from the input and attempt to parse it as a u32 (unsigned 32-bit integer)
        let input: u32 = match input.trim().parse() {
            // If parsing is successful, assign the parsed number to 'input'
            Ok(num) => num,
            // If parsing fails, print an error message and continue to the next iteration of the loop
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };
        // Check if the user's guess matches the random number
        if input == random_number {
            // If the guess is correct, break out of the loop
            break;
        }
        // If the guess is incorrect, print a message and continue the loop
        println!("Sorry, that was wrong. Try again.");
    }
    // Print a message indicating that the user guessed correctly
    println!("Correct!");
}
