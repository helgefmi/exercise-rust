pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = n;

    // Handle even numbers separately, to speed up the next part.
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    // Test 3, 5, 7, ..
    let mut candidates = (3..).step_by(2);
    while n > 1 {
        let d = candidates.next().unwrap();
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
    }

    factors
}
