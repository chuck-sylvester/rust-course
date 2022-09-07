// import resources so that we can more easily use them
use std::io::stdin;

// create structure for list of friends
#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    your_name
        .trim()
        .to_lowercase()
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("Hello, {:?}", name);

    // define array of Visitor structures to hold list of allowed visitors
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge"),
        Visitor::new("fred", "Wow, who invited Fred?:"),
    ];

    let known_vistor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);

    match known_vistor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("You are not on the visitor list. Please leave.")
    }

}
