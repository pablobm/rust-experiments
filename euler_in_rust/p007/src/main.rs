fn main() {
    let target = 10_001;
    let mut primes = PrimeSieve::new();
    let mut candidate = 2;
    while primes.len() < target {
        if primes.test(candidate) {
            primes.add(candidate);
        }
        candidate += 1;
    }
    println!("{}", primes.largest());
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
