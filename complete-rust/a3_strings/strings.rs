// The Complete Rust Programming Reference Guide
// strings.rs

fn main() {
    let question = "How are you?";              // &str type
    let person: String = "Bob".to_string();
    let namaste = String::from("✬");            // unicode (I think)

    dbg!("{} {} {}", &question, &person, &namaste);

    println!();
    println!("{}{}{} {}\n", &namaste, &person, &namaste, &question);

    dbg!("{}{}{} {}", &question, &person, &namaste);

}