fn main() {
    let mut prev = 1;
    let mut next = 2;
    let mut acc = 0;
    let mut tmp;
    while next < 4000000 {
        if next % 2 == 0 {
            acc += next;
        }
        tmp = prev;
        prev = next;
        next = next+tmp;
    }
    println!("{}", acc);
}
