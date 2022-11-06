use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..=100);

    println!("{}, {}", secret_number, 123123);
    println!("Please input your guess.");
}
