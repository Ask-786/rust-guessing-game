use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::{io, u32};

fn main() {
    let randome_number = rand::thread_rng().gen_range(1..101);
    let mut message = "Please enter a number".normal();
    let mut score: u32 = 0;

    println!(
        "{}",
        format!(
            "You have to guess a number, and we will guide you if the
number you entered is higher or lesser.
and you will get a result based on your number of guesses,
{}",
            "The lesser, the better. Let's start!!".yellow()
        )
    );

    loop {
        let mut guess = String::new();

        println!("{}\n", message);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed reading the input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                message = "Invalid input, try again".normal();
                continue;
            }
        };

        score += 1;

        match guess.cmp(&randome_number) {
            Ordering::Less => message = "That's too low, try again".red(),
            Ordering::Greater => message = "That's a bit greater, try again".red(),
            Ordering::Equal => {
                let score = score.to_string();
                println!(
                    "{}",
                    format!("\nCongratulations You got it!!, Your Score: {}", score).green()
                );
                break;
            }
        }
    }
}
