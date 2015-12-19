pub struct PrimeSieve {
    ordered_known_primes: Vec<u64>,
}

impl PrimeSieve {
    pub fn new() -> PrimeSieve {
        PrimeSieve {
            ordered_known_primes: vec![2],
        }
    }

    /// Returns the prime factors of `target`, in order.
    /// Each factor will appear as many times as it can
    /// divide `target`
    ///
    /// # Examples
    ///
    /// ```
    /// use euler::prime_sieve::PrimeSieve;
    ///
    /// let factors = PrimeSieve::new().factors_of(6);
    /// assert_eq!(factors.len(), 2);
    /// assert_eq!(factors[0], 2);
    /// assert_eq!(factors[1], 3);
    ///
    /// let factors = PrimeSieve::new().factors_of(9);
    /// assert_eq!(factors.len(), 2);
    /// assert_eq!(factors[0], 3);
    /// assert_eq!(factors[1], 3);
    ///
    /// let factors = PrimeSieve::new().factors_of(26);
    /// assert_eq!(factors.len(), 2);
    /// assert_eq!(factors[0], 2);
    /// assert_eq!(factors[1], 13);
    ///
    /// let factors = PrimeSieve::new().factors_of(45);
    /// assert_eq!(factors.len(), 3);
    /// assert_eq!(factors[0], 3);
    /// assert_eq!(factors[1], 3);
    /// assert_eq!(factors[2], 5);
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
        self.extend_known_primes_up_to(target);
        if self.ordered_known_primes.iter().any(|prime| *prime == target) {
            vec![target]
        }
        else {
            self.collect_factors_from_known_primes(target)
        }
    }

    fn extend_known_primes_up_to(&mut self, target: u64) {
        for candidate in self.largest_known_prime()+1..target+1 {
            let mut reduction = candidate;
            for prime in self.ordered_known_primes.iter() {
                while reduction % prime == 0 {
                    reduction = reduction / prime;
                }
            }
            if reduction == candidate {
                self.ordered_known_primes.push(candidate);
            }
        }
    }

    fn collect_factors_from_known_primes(&self, target: u64) -> Vec<u64> {
        let mut factors = Vec::new();
        let mut reduction = target;
        for &prime in self.ordered_known_primes.iter() {
            while reduction % prime == 0 {
                factors.push(prime);
                reduction = reduction / prime;
            }
        }
        factors
    }

    fn largest_known_prime(&self) -> &u64 {
        self.ordered_known_primes.last().unwrap()
    }
}
