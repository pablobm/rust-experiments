extern crate euler;

use euler::prime_sieve::PrimeSieve;

fn main() {
    let target = 10_001;
    let mut primes = PrimeSieve::new();
    let mut candidate = 2;
    while primes.primes.len() < target {
        primes.test(candidate);
        candidate += 1;
    }
    println!("{}", primes.largest());
}
