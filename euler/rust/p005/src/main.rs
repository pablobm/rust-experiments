extern crate euler;

use std::collections::HashMap;
use euler::factors_of;

fn main() {
    let target = 20;
    let mut counts = HashMap::new();
    for i in 2..target+1 {
        let mut tmp_counts = HashMap::new();
        for factor in factors_of(i) {
            let old_count = match tmp_counts.get(&factor) {
                Some(c) => *c,
                None => 0,
            };
            tmp_counts.insert(factor, old_count+1);
        }
        for (factor, tmp_count) in tmp_counts {
            let count = match counts.get(&factor) {
                Some(c) => *c,
                None => 0,
            };
            counts.insert(factor, (tmp_count as f64).max(count as f64) as u64);
        }
    }
    let mut acc = 1;
    for (factor, count) in counts {
        println!("{}^{}", factor, count);
        acc = acc * factor.pow(count as u32);
    }
    println!("{}", acc);
}
