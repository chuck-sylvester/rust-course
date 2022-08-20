// The Complete Rust Programming Reference Guide
// greet.rs

use std::env;

fn main() {
    let program_name = env::args().next();
    println!("\nValue of program_name: {:?}", program_name);

    let my_name = env::args().skip(1).next();
    println!("     Value of my_name: {:?}\n", my_name);

    match my_name {
        Some(n) => println!("Hi there, {}\n", n),
        None    => panic!("Didn't receive any name?")
    }
}