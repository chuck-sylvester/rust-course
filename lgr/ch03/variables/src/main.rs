#![allow(unused)]

use colored::Colorize;

fn main() {
    let mut x = 5;
    println!("{}{}", "The value of x is: ".yellow(), x);

    x = 6;
    println!("{}{}", "The value of x is: ".yellow(), x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    another_function(5);
}

fn another_function(x: u32) {
    println!("Another function and the integer: {}", x);
}
