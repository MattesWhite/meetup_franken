/// Returns a Vec with n-elements.
/// The indexes with `true`-Values are primes.
fn prime_time(n: usize) -> Vec<bool> {
    assert!(n >= 2);

    // create Vec -> assumption all numbers are primes
    let mut numbers = vec![true; n];
    numbers[0] = false;
    numbers[1] = false;

    // start at first prime (2)
    let mut active = 2usize;

    // Repeat algorithm until end of Vec
    while active < n {
        // Search for the next prime
        while active < n && !numbers[active] {
            active += 1;
        }
        
        // Set multiples of the found prime to `false`
        for i in (active * 2..n).step_by(active) {
            numbers[i] = false;
        }
        
        active += 1;
    }

    numbers
}

fn main() {
    let numbers = prime_time(21);
    numbers.iter().enumerate().for_each(|(idx, active)| {
        println!("Number {} is a prime ({})", idx, *active);
    });
}
