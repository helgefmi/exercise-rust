fn pluralize_bottles(n: u32, cap_first: bool) -> String {
    match n {
        0 => if cap_first {
            "No more bottles"
        } else {
            "no more bottles"
        }
        .to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n),
    }
}

pub fn verse(n: u32) -> String {
    let first = format!(
        "{} of beer on the wall, {} of beer.",
        pluralize_bottles(n, true),
        pluralize_bottles(n, false)
    );
    let last = match n {
        0 => "Go to the store and buy some more, 99 bottles of beer on the wall.".to_string(),
        _ => format!(
            "Take {} down and pass it around, {} of beer on the wall.",
            if n == 1 { "it" } else { "one" },
            pluralize_bottles(n - 1, false)
        ),
    };
    first + "\n" + &last + "\n"
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
