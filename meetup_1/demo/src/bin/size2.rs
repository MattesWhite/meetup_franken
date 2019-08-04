use demo::{MyError, print_size};
use std::error::Error;

fn main() {
    println!();
    print_size!(String, 40);
    print_size!(MyError, 40);
    print_size!(Box<dyn Error>, 40);
    println!("-------------------------------------------");
    print_size!(Result<String, MyError>, 40);
    print_size!(Result<String, Box<dyn Error>>, 40);
    print_size!(Result<(), MyError>, 40);
    print_size!(Result<(), Box<dyn Error>>, 40);
    println!();
}
