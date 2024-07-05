use std::io::{self, Write}; // Importing the io module from the standard library, including the Write trait

fn main() { // Defining the main function, the entry point of the program

    print!("Please enter your name: "); // Printing a prompt message to the console without a newline
    io::stdout().flush().unwrap(); // Flushing the output buffer to ensure the prompt message is displayed immediately
    let mut name = String::new(); // Creating a mutable String variable to store the user's input
    io::stdin().read_line(&mut name).expect("Failed to read line"); // Reading a line of input from the user and storing it in the 'name' variable, handling any potential errors
    let name = name.trim_end(); // Removing any trailing whitespace or newline characters from the input

    
    println!("Hello, {}!", name); // Printing a greeting message that includes the user's name
}
