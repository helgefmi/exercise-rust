use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&x| *x > 0)
        .flat_map(|n| (*n..limit).step_by(*n as usize).collect::<Vec<u32>>())
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}
