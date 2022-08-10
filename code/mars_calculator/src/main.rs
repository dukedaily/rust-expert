use std::io;

fn main() {
    println!("Enter a weight:(kg)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    println!("weight: {}", weight);
    dbg!(weight);

    println!("input: {}", input);
    // let mars_weight = mars_calculator(100.9); // error
    let mut mars_weight = mars_calculator(weight);
    mars_weight = mars_weight * 1000.0;

    println!("Weight on Mars: {}kg", mars_weight);
}

fn mars_calculator(_weight: f32) -> f32 {
    (_weight / 9.81) * 3.711
}
