fn prime_time(n: usize) -> Vec<bool> {
    assert!(n >= 2);

    let mut numbers = vec![true; n];
    numbers[0] = false;
    numbers[1] = false;

    let mut active = 2usize;
    while active < n {
        println!("numbers: {:?}", &numbers);
        while active < n && !numbers[active] {
            active += 1;
        }
        println!("working with {}", active);
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
