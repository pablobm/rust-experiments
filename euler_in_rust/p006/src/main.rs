fn main() {
    let target : u64 = 100;
    let mut acc;

    acc = 0;
    for i in 1..target+1 {
        acc = acc + i.pow(2);
    }
    let sum_of_sq = acc;
    println!("Sum of squares: {}", sum_of_sq);

    acc = 0;
    for i in 1..target+1 {
        acc = acc + i;
    }
    let sq_of_sum = acc.pow(2);
    println!("Square of the sum: {}", sq_of_sum);

    println!("{}", sq_of_sum - sum_of_sq);
}
