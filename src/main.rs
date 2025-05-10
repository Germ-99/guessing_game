use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    if guess == secret_number {
        println!("Congrats, you guessed the correct number, which was: {}", secret_number);
    } else {
        println!("Wrong guess. The secret number was: {}", secret_number);
    }
}
