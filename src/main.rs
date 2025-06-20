use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number ( between 1 and 100)!");

    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(number) => number,
        Err(_) => continue,
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
                println!("You win!");
                break;
        }
    }

    }

    
}
