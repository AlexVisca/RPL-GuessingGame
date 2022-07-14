use rand::Rng;
use std::io;
// Main runtime
fn main() {
    println!("Guess the number!");

    let secret_int = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_int}");

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
