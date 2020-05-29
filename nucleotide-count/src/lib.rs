use std::iter::once;
use std::collections::HashMap;

const NUCLEOTIDES: &'static str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let all_chars = dna.chars().chain(once(nucleotide));
    match all_chars.filter(|&c| !NUCLEOTIDES.contains(c)).nth(0) {
        Some(c) => Err(c),
        None => Ok(dna.chars().filter(|&c| c == nucleotide).count()),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    for c in NUCLEOTIDES.chars() {
        match count(c, dna) {
            Ok(num) => { map.insert(c, num); },
            Err(chr) => return Err(chr)
        }
    }
    Ok(map)
}
