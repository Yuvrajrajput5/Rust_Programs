fn isPrime(n: u64) -> bool {
    if n <= 1 {
        return false; 
    }
    if n <= 3 {
        return true; 
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false; 
        }
        i += 6;
    }

    true
}

fn main() {
    let n = 12;

    if isPrime(n) {
        println!("Yes, {} is a Prime number.", n);
    } else {
        println!("No, {} is not a prime number.", n);
    }
}