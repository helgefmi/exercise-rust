use std::iter;

pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = vec![];

        for _ in 0..self.row_count {
            let row = match rows.last() {
                None => vec![1],
                Some(last_row) => {
                    let zipped = last_row.iter().zip(last_row.iter().skip(1));
                    let sums = zipped.map(|(a, b)| a + b);
                    std::iter::once(1).chain(sums).chain(iter::once(1)).collect()
                }
            };
            rows.push(row);
        }
        rows
    }
}
