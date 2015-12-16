pub struct PrimeSieve {
    ordered_known_primes: Vec<u64>,
}

impl PrimeSieve {
    pub fn new() -> PrimeSieve {
        PrimeSieve {
            ordered_known_primes: vec![2],
        }
    }

    /// Returns the prime factors of `target`.
    ///
    /// # Examples
    ///
    /// Prime factors are returned in order:
    ///
    /// ```
    /// use euler::prime_sieve::PrimeSieve;
    ///
    /// let factors = PrimeSieve::new().factors_of(9);
    /// assert_eq!(factors.len(), 1);
    /// assert_eq!(factors[0], 3);
    ///
    /// let factors = PrimeSieve::new().factors_of(45);
    /// assert_eq!(factors.len(), 2);
    /// assert_eq!(factors[0], 3);
    /// assert_eq!(factors[1], 5);
    /// ```
    ///
    /// Note that, when `target` is prime, it is returned
    /// as the only prime factor:
    ///
    /// ```
    /// use euler::prime_sieve::PrimeSieve;
    ///
    /// let factors = PrimeSieve::new().factors_of(7);
    /// assert_eq!(factors.len(), 1);
    /// assert_eq!(factors[0], 7);
    /// ```
    ///
    pub fn factors_of(&mut self, target: u64) -> Vec<u64> {
        let target_sqrt = (target as f64).sqrt() as u64;
        self.extend_known_primes_up_to(target_sqrt);
        self.collect_factors_from_known_primes(target)
    }

    fn extend_known_primes_up_to(&mut self, limit: u64) {
        for candidate in self.largest_known_prime()+1..limit+1 {
            let candidate_sqrt = (candidate as f64).sqrt() as u64;
            let mut is_prime = true;
            for prime in self.ordered_known_primes.iter() {
                if prime > &candidate_sqrt {
                    break
                }
                else if candidate % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                self.ordered_known_primes.push(candidate);
            }
        }
    }

    fn collect_factors_from_known_primes(&self, target: u64) -> Vec<u64> {
        let mut factors: Vec<u64> = Vec::new();
        let target_sqrt = (target as f64).sqrt() as u64;
        for prime in self.ordered_known_primes.iter() {
            if prime > &target_sqrt {
                break;
            } else if target % prime == 0 {
                factors.push(*prime);
            }
        }
        if factors.len() == 0 {
            factors.push(target);
        }
        factors
    }

    fn largest_known_prime(&self) -> &u64 {
        self.ordered_known_primes.last().unwrap()
    }
}
