use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    loop {
        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let secret_number = thread_rng().gen_range(1..=100);

        let parsed_guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, to next game\n");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        println!("The secret number is : {}", secret_number);

        match parsed_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
