use std::io;
use rand::Rng;


// prelude is the set of items that’s shipped by default with every rust program
// (without having to import anything)
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    
    println!("Please input your guess.");

    let mut guess = String::new(); // new is an associated function of the String type
    // what happens when you bind a new value? is it a new reference, or is it the same string reference that’s being changed?
    io::stdin() // stdin function from io module. returns an instance of std::io::Stdin (a handle to the standard input)
        .read_line(&mut guess) // method of the instance. reference to guess. references are immutable by default
        .expect("Failed to read line"); // read_line returns a Result value (an enum)
    // if the value is an Err, it will crash the program
    // if the value is an Ok, it will return the value held inside

    println!("You guessed: {guess}");
}
