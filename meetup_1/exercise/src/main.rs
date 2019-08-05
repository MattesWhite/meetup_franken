//! Maybe you remember this from 
//! [The Rust Book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).
//! 
//! # The game
//!
//! Aim of this game is to guess the right number the PC holds. The secret 
//! number will always be 1 up to 100. After each guess the PC will responde 
//! if its number is higher, lower or equal.
//!
//! # Exercise
//! 
//! As you can see in the code below, this is a beginer's example. The code
//! has no proper error handling. Your task is to refactor the code and to 
//! introduce proper error handling.
//! 
//! The behaviour should be as follow:
//! - Refactor the guessing of the user into a function `guess()` that returns
//!   a `Result<u32, E>`.
//! - `guess()` should return different errors depending on the cause (wrecked 
//!   `stdin`, wrong user input, guess out of bounds, etc.).
//! - If the error originates from a failed read from `stdin` the application
//!   should panic.
//! - Every other error, the error message should be displayed to the user.

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

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