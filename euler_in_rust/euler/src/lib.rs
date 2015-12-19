/// Returns the prime factors of `target`, in order.
/// Each factor will appear as many times as it can
/// divide `target`
///
/// # Examples
///
/// ```
/// use euler::factors_of;
///
/// let factors = factors_of(6);
/// assert_eq!(factors.len(), 2);
/// assert_eq!(factors[0], 2);
/// assert_eq!(factors[1], 3);
///
/// let factors = factors_of(9);
/// assert_eq!(factors.len(), 2);
/// assert_eq!(factors[0], 3);
/// assert_eq!(factors[1], 3);
///
/// let factors = factors_of(26);
/// assert_eq!(factors.len(), 2);
/// assert_eq!(factors[0], 2);
/// assert_eq!(factors[1], 13);
///
/// let factors = factors_of(45);
/// assert_eq!(factors.len(), 3);
/// assert_eq!(factors[0], 3);
/// assert_eq!(factors[1], 3);
/// assert_eq!(factors[2], 5);
/// ```
///
/// Note that, when `target` is prime, it is returned
/// as the only prime factor:
///
/// ```
/// use euler::factors_of;
///
/// let factors = factors_of(7);
/// assert_eq!(factors.len(), 1);
/// assert_eq!(factors[0], 7);
/// ```
///
pub fn factors_of(target: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let target_sqrt = (target as f64).sqrt() as u64;
    let mut reduction = target;
    for candidate_prime in 2..target_sqrt+1 {
        while reduction % candidate_prime == 0 {
            factors.push(candidate_prime);
            reduction = reduction / candidate_prime;
        }
    }
    if reduction != 1 {
        factors.push(reduction);
    }
    factors
}
