fn main() {
    let mut my_string = String::from("hello"); // create a String from a string literal using from()
    my_string.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", my_string); // This will print `hello, world!`

}

// Get the first word from a list of space separated words
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}