fn main() {
    let target : i64 = 1000;
    for a in 1..target {
        for b in a+1..target-a {
            let c = target - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                println!("{}", a * b * c);
                std::process::exit(0);
            }
        }
    }
}
