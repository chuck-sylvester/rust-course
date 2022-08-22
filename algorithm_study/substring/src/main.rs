// Find Substring algorithm
// Determine whether or not string b can be found in string a
//
// Run program from command line with two input parameter, e.g.
// $ cargo run string1 string2 

use std::env;   // Rust standard library, process environment module

fn main() {
    println!("\nFind substring algorithm (start)");

    // Grab, validate, store string values entered by user from the environment
    let string1 = env::args().nth(1).expect("Please provide string 1 as the first command line value");
    let string2 = env::args().nth(2).expect("Please provide string 2 as the second command line value");
    
    println!("{}, {}", string1, string2);
    println!("{:?}, {:?}", string1, string2);

    let length_1 = string1.len();
    let length_2 = string2.len();

    println!("The values of string1 and string2 are: {:?} ({} char) and {:?} ({} char)", string1, length_1, string2, length_2);

    // Check to see if string1 contains string2
    let mut rc = string1_contains_string2(&string1, &string2);

    // Print result of function call
    if rc {
        println!("rc = true");
    } else {
        println!("rc = false");
    }

    // Check to see if string1 can spell string2
    rc = string1_spells_string2(&string1, &string2);

    // Print result of function call
    if rc {
        println!("rc = true");
    } else {
        println!("rc = false");
    }

    println!("Find substring algorithm (end)\n");
}

// Algorithm to determine if string1 contains string2
fn string1_contains_string2(s1: &String, s2: &String) -> bool {
    println!("Enter function: string1_contains_string2()");
    true
    /*
    if a.contains(b) {
        println!("{a} contains {b}");
    } else {
        println!("{a} does not contain {b}");
    }

    if a.contains(c) {
        println!("{a} contains {c}");
    } else {
        println!("{a} does not contain {c}");
    }
    */
}

// Algorithm to determine if the letters in string1 can spell string2 
fn string1_spells_string2(s1: &String, s2: &String) -> bool {
    println!("Enter function: string1_spells_string2()");
    true
    /*
    if a.contains(b) {
        println!("{a} contains {b}");
    } else {
        println!("{a} does not contain {b}");
    }

    if a.contains(c) {
        println!("{a} contains {c}");
    } else {
        println!("{a} does not contain {c}");
    }
    */
}
