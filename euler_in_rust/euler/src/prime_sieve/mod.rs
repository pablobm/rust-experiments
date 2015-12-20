pub struct PrimeSieve {
    pub primes: Vec<u64>,
}

impl PrimeSieve {
    pub fn new() -> PrimeSieve {
        PrimeSieve {
            primes: vec![],
        }
    }

    pub fn test(&mut self, target: u64) -> bool {
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

    pub fn largest(&self) -> u64 {
        *self.primes.last().unwrap()
    }
}
