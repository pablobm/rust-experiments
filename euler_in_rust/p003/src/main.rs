use std::collections::LinkedList;

fn main() {
    let target : u64 = 600851475143;
    let target_sqrt = (target as f64).sqrt() as u64;
    let mut primes = LinkedList::new();
    let mut largest_factor = 1;

    for x in (2..target_sqrt) {
        let mut is_prime = true;
        let x_sqrt = (x as f64).sqrt() as u64;
        for prime in primes.iter() {
            if prime > &x_sqrt {
                break;
            } else if x % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push_back(x);
            if target % x == 0 {
                largest_factor = x;
            }
        }
    }

    println!("{}", largest_factor);
}
