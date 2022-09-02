fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 + 2 = {}", 1 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important
    println!("1 - 2 = {}", 1i8 - 2);
    println!("1 - 2 = {}", 1 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);    // I think this will be false
    println!("true OR false is {}", true || false);     // I think this will be true
    println!("NOT true is {}", !true);                  // I think this will be false

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
    println!("Two million is written as {}", 2_000_000);
}
