use demo::print_size;

fn main() {
    println!();
    print_size!(String);
    print_size!(Option<String>);
    print_size!(Option<Box<String>>);
    print_size!(Option<&String>);
    println!("---------------------------------");
    print_size!(u64);
    print_size!(Option<u64>);
    print_size!(Option<Box<u64>>);
    print_size!(Option<&u64>);
    println!();
}
