use std::process::exit;

fn main() {
    let target = 2_000_000;
    let mut sum = 0;
    let mut sieve = PrimeSieve::new();

    for i in 2..target {
        if sieve.test(i) {
            sum += i;
        }
    }
    println!("{}", sum);
}

struct PrimeSieve {
    primes: Vec<u64>,
}

impl PrimeSieve {
    fn new() -> PrimeSieve {
        PrimeSieve {
            primes: vec![],
        }
    }

    fn test(&mut self, target: u64) -> bool {
        let mut is_prime = true;
        let target_sqrt = (target as f64).sqrt() as u64;
        for prime in &self.primes {
            if target % *prime == 0 {
                is_prime = false;
                break;
            }
            if target_sqrt < *prime {
                break;
            }
        }
        if is_prime {
            self.primes.push(target);
        }
        is_prime
    }

    fn largest(&self) -> u64 {
        *self.primes.last().unwrap()
    }

    fn add(&mut self, prime: u64) {
        self.primes.push(prime);
    }

    fn len(&self) -> usize {
        self.primes.len()
    }
}
