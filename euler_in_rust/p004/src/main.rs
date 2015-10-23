fn main() {
    let mut best_fit : Option<u64> = None;
    let mut f1 = 0;
    let mut f2 = 0;
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
                    f1 = i;
                    f2 = j;
                    best_fit = Some(product);
                }
            }
        }
    }
    match best_fit {
        Some(x) => {
            println!("{} = {} * {}", x, f1, f2);
        },
        None => {
            println!("Not found :-(");
        },
    }
}

fn is_palindromic(number: u64) -> bool {
  let as_string = number.to_string();
  let digits : Vec<_> = as_string.chars().collect();
  let mut is_it = true;
  for i in 0..(digits.len()/2) {
      let left = digits[i];
      let right = digits[digits.len()-1-i];
      if left != right {
          is_it = false;
      }
  }
  is_it
}
