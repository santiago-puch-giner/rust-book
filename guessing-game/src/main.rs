use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    // Generate a random number to be guessed
    let random_number: i32 = rand::rng().random_range(1..=100);

    loop {
        // Get guess from command line
        println!("Please input your guess: ");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.trim().to_string();

        // Exit loop with "exit" input
        if guess == "exit" {
            println!("Exiting program...");
            break;
        }

        // Try to parse number
        let guess_num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Unrecognized number");
                continue;
            }
        };

        // Compare guess with secret number
        match guess_num.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!(
                    "You guessed it, you bastard! The number was indeed {random_number}. Well done!"
                );
                break;
            }
        }
    }
}
