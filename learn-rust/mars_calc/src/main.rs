// Calculate your weight on Mars
// Originally based on metric weight
// Updated by CPS to work with pounds

use std::io;

fn main() {
    println!("Enter your weight on earth in pounds: ");
    let mut user_input = String::new();
    dbg!(&user_input);

    io::stdin().read_line(&mut user_input).unwrap();
    dbg!(&user_input);

    let weight: f32 = user_input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);
    dbg!(mars_weight);
    println!("Weight on Mars: {}lbs", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}