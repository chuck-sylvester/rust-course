// The Rust Programming Language Book
// Chapter 5 - Using Structs

#[derive(Debug)] // run macro to enable debug printing of User
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    println!("*** Enter: build_user() ***");

    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("chuck.sylvester@cs.com"),
        username: String::from("cps001"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1 = {:#?}", user1);

    let name1: String = user1.username;
    println!("user1.username = {:#?}", name1);

    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    
    println!("user2 = {:#?}", user2);

    user1.username = String::from("wallice 456");
    println!("user1 = {:#?}", user1);
}
