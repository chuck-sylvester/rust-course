// Let's Get Rusty
// Programming a guessing game in Rust

// extern crate colored;

use std::io;
use std::cmp::Ordering;
use colored::*;
use rand::Rng;

fn main() {
    println!("Msg: {}", "Guess the number!".blue());

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Msg: {}{}", "The secret number is: ".blue(), "*secret*");

    loop {
        println!("Msg: {}", "Please input your guess.".blue());

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        // Shadow and recast "guess" variable as integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Msg: {}{}", "You guessed: ".blue(), guess);
    
        // Handle the possible return values of comparing two numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}