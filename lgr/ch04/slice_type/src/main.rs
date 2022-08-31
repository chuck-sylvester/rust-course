// Chapter 4 - Working with String Slices
// Write a function that takes a string of words separated by spaces and returns the first word
// it finds in the string. If the function doesn’t find a space in the string, the whole string
// must be one word, so the entire string should be returned.

fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello world!");

    let word = first_word(&s);  // word will get the value 5
    println!("Value of word is: {}", word);

    s.clear();  // empty the string, making it equl to ""

    // Now experiment wiht the dbg! macro
    println!("Experimenting with the dbg! macro...");
    let a = 2;
    let b = dbg!(a * 2) + 1;
    assert_eq!(b, 5);
    let result = dbg!(1+1) + 3;
    println!("The value of result is: {}", result);

}

// return the index of the end of the first word, indicated by a space
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    dbg!(bytes[0], bytes[1], bytes[2], bytes[3], bytes[4]);

    for (i, &item) in bytes.iter().enumerate() {
        dbg!(item);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
