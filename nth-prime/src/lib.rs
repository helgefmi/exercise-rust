fn is_prime(n: &u32) -> bool {
    // Handle special case of n being divisible by 2.
    if *n == 2 {
        return true;
    } else if *n % 2 == 0 {
        return false;
    }

    // Only iterate up to the square root of n.
    let iter_to = ((*n as f64).sqrt() as u32) + 1;

    // Checks only odd numbers, due to the special case handling above.
    !(3..iter_to).step_by(2).any(|d| n % d == 0)
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(is_prime).nth(n as usize).unwrap()
}
