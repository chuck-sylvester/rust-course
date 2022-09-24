fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}\n");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}\n");

    let spaces = "   ";
    println!("spaces: [{}]", spaces);
    let spaces = spaces.len();
    println!("spaces: [{}]\n", spaces);

    another_function(5);

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(50);
    println!("The value of y is: {y}");

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}

fn another_function(x: i32) {
    println!("You have entered another_function()");
    println!("The argument sent sent is: {x}");
    println!("You are now leaving another_function()\n");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
