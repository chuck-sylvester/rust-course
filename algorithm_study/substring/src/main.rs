// Find Substring algorithm
// Determine whether or not string b can be found in string a

// First attempt will be brute force
//  - create a and b string variables
//  - set values for a and b, e.g. "apple" and "ape"
//  - maybe get the string length of both and use to iterate
//  - then, will need outer and inner for loops
//  - issue: string components are not bytes, rather they are longer unicode
//  - so, I may take the easy way and use the Rust "contains" method on a string

fn main() {
    println!("\nFind substring algorithm\n");

    let a = "apple";
    let b = "ape";
    let c = "ppl";

    let length_a = a.len();
    let length_b = b.len();
    let length_c = c.len();

    // let mut current_letter_found = false;

    println!("The values of a and b are: {} ({} char) and {} ({} char)\n", a, length_a, b, length_b);
    println!("The values of a and c are: {} ({} char) and {} ({} char)\n", a, length_a, c, length_c);

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


}
