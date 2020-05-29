pub fn reply(message: &str) -> &str {
    let message = message.trim();

    let letters: String = message.chars().filter(|x| x.is_alphabetic()).collect();
    let is_uppercase = !letters.is_empty() && letters.chars().all(|x| x.is_uppercase());
    let is_question = message.ends_with('?');

    match message {
        x if x.is_empty() => "Fine. Be that way!",
        _ if is_uppercase && is_question => "Calm down, I know what I'm doing!",
        _ if is_uppercase => "Whoa, chill out!",
        _ if is_question => "Sure.",
        _ => "Whatever.",
    }
}
