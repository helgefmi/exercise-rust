use std::collections::HashMap;

const VALID_DNA: &'static str = "ACGT";
const VALID_RNA: &'static str = "ACGU";

fn find_first_mismatched_char(input: &str, valid: &str) -> Option<(usize, char)> {
    input.chars().enumerate().find(|(_, c)| !valid.contains(*c))
}

#[derive(Debug, PartialEq)]
pub struct DNA {
    dna: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        match find_first_mismatched_char(dna, VALID_DNA) {
            Some((i, _)) => Err(i),
            None => Ok(DNA {
                dna: dna.to_string(),
            }),
        }
    }

    pub fn into_rna(self) -> RNA {
        let tuples = vec![('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')];
        let map: HashMap<char, char> = tuples.into_iter().collect();
        RNA {
            rna: self.dna.chars().map(|c| map[&c]).collect(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    rna: String,
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        match find_first_mismatched_char(rna, VALID_RNA) {
            Some((i, _)) => Err(i),
            None => Ok(RNA {
                rna: rna.to_string(),
            }),
        }
    }
}
