fn matching_brackets(a: char, b: char) -> bool {
    [('(', ')'), ('[', ']'), ('{', '}')].contains(&(a, b))
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => match stack.pop() {
                Some(top) => if !matching_brackets(top, c) { return false; }
                None => return false
            },
            _ => {}
        }
    }
    stack.is_empty()
}
