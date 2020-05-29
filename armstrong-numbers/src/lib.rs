pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();

    let sum = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|i| i.pow(num_str.len() as u32))
        .sum::<u32>();

    sum == num
}
