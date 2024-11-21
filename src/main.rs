use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number ");
    let secret = rand::thread_rng().gen_range(1..=10);
    println!("The secret number is: {}", secret);
    println!("Choose a number from 1-10: ");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("You guessed: {}", guess);
    match guess.cmp(&secret) {
        Ordering::Less => println!("Come on champ, you are a little step away from it"),
        Ordering::Greater => println!("No no, so slow, you are much ahead of it"),
        Ordering::Equal => println!("Congo Champ! You guessed it right"),
    }
}
