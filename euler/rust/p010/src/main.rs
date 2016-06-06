extern crate euler;

use euler::prime_sieve::PrimeSieve;

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
