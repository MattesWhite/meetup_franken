use demo::{PubError, print_size};
use std::error::Error;

fn main() {
    println!();
    print_size!(String, 40);
    print_size!(PubError, 40);
    print_size!(Box<dyn Error>, 40);
    println!("-------------------------------------------");
    print_size!(Result<String, PubError>, 40);
    print_size!(Result<String, Box<dyn Error>>, 40);
    print_size!(Result<(), PubError>, 40);
    print_size!(Result<(), Box<dyn Error>>, 40);
    println!();
}
