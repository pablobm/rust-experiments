fn main() {
    let mut best_fit : Option<u64> = None;
    for i in (100..1000).rev() {
        match best_fit {
            Some(x) => if i*999 < x {
              break;
            },
            None => (),
        }
        for j in (i..1000).rev() {
            let product = i * j;
            if is_palindromic(product) {
                let found = match best_fit {
                    None => true,
                    Some(x) => x < product,
                };
                if found {
                    best_fit = Some(product);
                }
            }
        }
    }
    match best_fit {
        Some(x) => {
            println!("{}", x);
        },
        None => {
            println!("Not found :-(");
        },
    }
}

fn is_palindromic(number: u64) -> bool {
  let as_string = number.to_string();
  let digits : Vec<_> = as_string.chars().collect();
  for i in 0..(digits.len()/2) {
      let left = digits[i];
      let right = digits[digits.len()-1-i];
      if left != right {
          return false;
      }
  }
  true
}
