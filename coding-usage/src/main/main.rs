use coding_common::{greet::greet, math::add};

fn main() {
    let sum = add(2, 3);
    println!("2 + 3 = {}", sum);

    let message = greet::hello("Rust");
    println!("{}", message);
}