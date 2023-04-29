// bring in the standard io library
use std::io;
use std::cmp::Ordering;
// bring in the Rng trait from rand to get methods for random numbers
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // define a variable that can be mutatable
        let mut guess = String::new();
    
        // attempt to read in line from the standard input and store it in the guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // we are using shadowing here on guess so we can reuse the variable since we are just converting it
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!!");
                break;
            }
        }
    }
}
