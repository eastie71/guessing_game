use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the Number game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut num_guesses = 1;
    
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("");
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)   // &mut indicates a mutable reference
            .expect("Failed to read line");

        // Unhandled conversion will crash out here if guess is NAN
        // let guess: u32 = guess.trim().parse().expect("Please type a number between 1 and 100");

        // Handle conversion without crashing out
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number between 1 and 100");
                continue;
            }
        };

        println!("Guess #: {}, You guessed: {}", num_guesses, guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You got it!!");
                println!("Number of guesses was: {}", num_guesses);
                break;
            }
        }

        num_guesses += 1;
    }
    
}

