#![allow(unused)]

fn main() {
    let mut s = String::from("hello");
    s += ", world!";
    s.push_str(" What's new?");
    println!("s = {}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

}