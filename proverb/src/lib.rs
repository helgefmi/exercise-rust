pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        String::new()
    } else {
        (0..list.len())
            .map(|n| {
                if n == list.len() - 1 {
                    format!("And all for the want of a {}.", list[0])
                } else {
                    format!("For want of a {} the {} was lost.", list[n], list[n + 1])
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}
