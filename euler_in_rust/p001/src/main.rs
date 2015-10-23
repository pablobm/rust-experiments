fn main() {
    let mut acc = 0;
    for x in 0..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            acc += x;
        }
    }
    print!("{}\n", acc);
}
