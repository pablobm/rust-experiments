extern crate euler;

use euler::prime_sieve::PrimeSieve;

pub fn main() {
    let target = 600851475143;
    let mut sieve = PrimeSieve::new();
    let prime_factors = sieve.factors_of(target);

    match prime_factors.last() {
        Some(x) => println!("{}", x),
        None => println!("{} is prime!", target),
    }
}
