//! Maybe you remember this from 
//! [The Rust Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).
//! 
//! # The game
//!
//! Aim of this game is to guess the right number the PC holds. The secret 
//! number will always be 1 up to 100. After each guess the PC will responde 
//! if its number is higher, lower or equal.
//!
//! # Solution
//! This is my personal solution to the task statet in the exercise.

use rand::Rng;
use snafu::{ResultExt, Snafu};
use std::cmp::Ordering;
use std::io;

/// A custom error, createt by usage of the [`snafu`](https://crates.io/crates/snafu) 
/// crate.
#[derive(Debug, Snafu)]
enum MyError {
    /// This error occures if a read from `stdin` fails. This error is rather 
    /// sever because it indicates a closed file descriptor.
    #[snafu(display("Failed to read line: {}", source))]
    FailedRead { source: io::Error },
    /// Occures when the user input can not be interpreted as an integer.
    #[snafu(display("Failed to interpret {}: {}", guess.trim(), source))]
    InvalidInput {
        source: std::num::ParseIntError,
        guess: String,
    },
    /// The guessed number musst be within the interval [1, 100]. If the guess
    /// is greater or zero this error is returned.
    #[snafu(display("Input out of the number's range [1, 100]"))]
    OutOfRange,
}

/// Produces a new guess from user input.
/// 
/// # Errors
/// 
/// This function fails if the input can not be read from `stdin`, the input is
/// not an integer or if the input is outside the valid interval [1, 100].
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

/// See module description for further information.
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
