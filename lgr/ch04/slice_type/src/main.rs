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


}

// return the index of the end of the first word, indicated by a space
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    dbg!(bytes[0]);
    dbg!(bytes[1]);
    dbg!(bytes[2]);
    dbg!(bytes[3]);
    dbg!(bytes[4]);

    for (i, &item) in bytes.iter().enumerate() {
        dbg!(item);
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


