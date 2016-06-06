extern crate euler;

use euler::factors_of;

pub fn main() {
    let target = 600851475143;
    let prime_factors = factors_of(target);

    match prime_factors.last() {
        Some(x) => println!("{}", x),
        None => println!("{} is prime!", target),
    }
}
