fn main() {
    // In general, the `{}` will be automatically replaced with any arguments.
    // These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can invoked by specified format character after a
    // `:`.
    println!("Base 10 repr:               {}",   69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>5}", number=1);

    // You can use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`.
    // User-defined types do not implement fmt::Display by default

    #[allow(dead_code)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement fmt::Display
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    // For Rust 1.58 and above, you can directly capture the argument from surrounding variable.
    // Just like the above, this will output "     1". 5 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}");

    // Activity
    // Add a println! macro call that prints: "Pi is roughly 3.142" by controlling the number of decimal places shown.
    // For the purposes of this exercise, use "let pi = 3.141592" as an estimate for pi.
    // Hint: you may need to check the std::fmt documentation for setting the number of decimals to display.

    println!("----------");
    let pi = 3.14159265358979323846;
    println!("Pi is roughly {}", pi);
    println!("Pi is roughly {:.1}", pi);
    println!("Pi is roughly {:.2}", pi);
    println!("Pi is roughly {:.3}", pi);
    println!("Pi is roughly {:.4}", pi);
    println!("Pi is roughly {:.5}", pi);
    println!("----------");
}
