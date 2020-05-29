pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut sieve = vec![false; (upper_bound + 1) as usize].into_boxed_slice();
    let mut primes = Vec::new();
    for i in 2..=upper_bound {
        if sieve[i as usize] == false {
            primes.push(i as u64);
            for i2 in (i..=upper_bound).step_by(i as usize) {
                sieve[i2 as usize] = true;
            }
        }
    }
    primes
}
