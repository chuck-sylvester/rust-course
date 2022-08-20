// The Complete Rust Programming Reference Guide
// functions.rs

fn main() {
    let a: u64 = 17;
    dbg!(a);

    let b = 3;
    dbg!(b);
    
    let result = add(a, b);
    println!("Result {}", result);

    let score = 2048;
    println!("score in main() before calling increase_by(): {}", score);

    increase_by(score, 30);
    println!("score in main() after calling increase_by(): {}", score);
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn increase_by(mut val: u32, how_much: u32) {
    val += how_much;
    println!("You made {} points", val);
}