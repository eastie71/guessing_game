use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the Number game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("The secret number is: {}", secret_number);

    println!("Please enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)   // &mut indicates a mutable reference
        .expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse().expect("Please type a number between 1 and 100");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You got it!!")
    }
}

