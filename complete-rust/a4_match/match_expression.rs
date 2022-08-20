// The Complete Rust Programming Reference
// match_expression.rs

fn main() {
    let status = req_status();
    dbg!(status);

    match status {
        200 => println!("Success"),
        404 => println!("Not Found"),
        other => println!("Request failed with code: {}", other)
    }
}

fn req_status() -> u32 {
    200
}