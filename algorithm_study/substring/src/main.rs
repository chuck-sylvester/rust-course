// Find Substring algorithm
// Determine whether or not string b can be spelled from the letters in string a
// Run program from command line with two input parameter, e.g.
// $ cargo run apple ape 

// Silence some warnings (for now)
#![allow(dead_code, unused_mut, unused_variables)]

// Create local name bindings synonymous with some other path.
// Usually used to shorten the path required to refer to a module item.
// These declarations may appear in modules and blocks, usually at the top.
//
use std::env;   // Inspection and manipulation of the process’s environment

fn main() {
    println!("\n*** Enter function: main() ***");

    // Grab, validate, store string values entered by user from the environment
    let string1 = env::args().nth(1).expect("Please provide string 1 as the first command line value");
    let string2 = env::args().nth(2).expect("Please provide string 2 as the second command line value");
    println!("{}, {}", string1, string2);
    
    let length_1 = string1.len();
    let length_2 = string2.len();
    println!("The values of string1 and string2 are: {:?} ({} char) and {:?} ({} char)", string1, length_1, string2, length_2);

    // Check to see if string1 contains string2 as a substring
    let mut rc = string1_contains_string2(&string1, &string2);

    if rc {
        println!("rc = true");
    } else {
        println!("rc = false");
    }

    // Check to see if the letters in string1 can be used to spell string2
    rc = string1_spells_string2(&string1, &string2);

    // Print result of function call
    if rc {
        println!("rc = true");
    } else {
        println!("rc = false");
    }

    println!("\n*** Exit function: main() ***");
}

// Determine whether string1 contains string2
fn string1_contains_string2(s1: &String, s2: &String) -> bool {
    println!("\n*** Enter function: string1_contains_string2() ***");

    if s1.contains(s2) {
        println!("{} contains {}", s1, s2);
        true
    } else {
        println!("{} does not contain {}", s1, s2);
        false
    }
}

// Algorithm to determine if the letters in string1 can spell string2 
fn string1_spells_string2(s1: &String, s2: &String) -> bool {
    println!("\n*** Enter function: string1_spells_string2() ***");

    // Loop through each letter in s2 to see if it can be found in s1
    // Do this by creating an inner loop with each value of s2 to iterate through s1
    // For now, letters in s1 can be reused
    // You may need to "parse" the strings to bytes
 
    

    true
}
