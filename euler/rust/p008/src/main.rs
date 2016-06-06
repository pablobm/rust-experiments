use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut target_file = match File::open("digits.txt") {
        Err(err) => panic!("Error opening file: {}", err),
        Ok(file) => file,
    };
    let mut target_string = String::new();
    match target_file.read_to_string(&mut target_string) {
        Err(err) => panic!("Error reading file: {}", err),
        Ok(_) => {},
    }
    let target_digits = target_string.chars()
        .filter(|char| char.is_digit(10))
        .map(|char| u64::from_str_radix(&char.to_string(), 10).unwrap());
    let target_len = 13;
    let mut queue = VecDeque::new();
    let mut max = 0;

    for digit in target_digits {
        if queue.len() == target_len {
            queue.pop_front();
        }
        queue.push_back(digit);
        let product = queue.iter().fold(1, |acc, &item| acc * item);
        if product > max {
            max = product;
        }
    }

    println!("{}", max);
}
