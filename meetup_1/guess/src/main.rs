//! # The game
//!
//! Aim of this game is to guess the right number the PC holds.
//! The secret number will always be 1 up to 100. After each
//! guess the PC will responde if its number is higher, lower or
//! equal.
//!
//! Maybe you remember this from the Rust Book:
//! https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
//!
//! # The task
//!
//! This code makes an extensive use of panics. Time to rewrite the
//! error handling to use a custom error type!

use rand::Rng;
use snafu::{ResultExt, Snafu};
use std::cmp::Ordering;
use std::io;

#[derive(Debug, Snafu)]
enum MyError {
    #[snafu(display("Failed to read line: {}", source))]
    FailedRead { source: io::Error },
    #[snafu(display("Failed to interpret {}: {}", guess.trim(), source))]
    InvalidInput {
        source: std::num::ParseIntError,
        guess: String,
    },
    #[snafu(display("Input out of the number's range [1, 100]"))]
    OutOfRange,
}

fn guess() -> Result<u32, MyError> {
    println!("Please input your guess: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).context(FailedRead)?;
    let guess: u32 = guess.trim().parse().context(InvalidInput { guess })?;
    if guess < 1 || guess > 100 {
        OutOfRange.fail()
    } else {
        Ok(guess)
    }
}

fn main() {
    println!("Guess the number!");
    println!("-----------------\n");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        match guess() {
            Ok(guess) => {
                print!("You guessed: {:3} ---> ", guess);
                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("*** You win! ***");
                        break;
                    }
                }
            }
            Err(MyError::FailedRead { source }) => {
                panic!("stdin seams to be broken: {}", source);
            }
            Err(e) => {
                println!("Invalid guess: {}", e);
            }
        };
    }
}
