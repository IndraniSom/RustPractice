use rand::Rng;
use std::io;
fn main() {
    println!("Guess the number ");
    let secret = rand::thread_rng().gen_range(1..=10);
    println!("The secret number is: {}", secret);
    println!("Choose a number from 1-10: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //error handling
    println!("You guessed : {}", guess);
}
