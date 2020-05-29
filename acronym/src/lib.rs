pub fn abbreviate(phrase: &str) -> String {
    let mut ret = String::new();
    let words = phrase.split(|c: char| !c.is_alphabetic() && c != '\'');
    for word in words {
        if word.is_empty() {
            continue;
        } else if word.ends_with(':') {
            let i = word.len() - 1;
            return word[..i].to_string();
        } else if word.chars().all(char::is_uppercase) || word.chars().all(char::is_lowercase) {
            ret.push(word.chars().next().unwrap());
        } else {
            ret.push_str(
                &word
                    .chars()
                    .into_iter()
                    .filter(char::is_ascii_uppercase)
                    .collect::<String>(),
            );
        }
    }

    ret.to_ascii_uppercase()
}
