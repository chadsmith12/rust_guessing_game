// bring in the standard io library
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // define a variable that can be mutatable
    let mut guess = String::new();

    // attempt to read in line from the standard input and store it in the guess
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
