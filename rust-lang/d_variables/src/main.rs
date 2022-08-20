// Working with Variables

fn main() {
  let mut x = 5;
  println!("The value of x is: {x}");

  x += 1;
  println!("The value of x is: {x}");

  x -= 2;
  println!("The value of x is: {x}");

  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
}
