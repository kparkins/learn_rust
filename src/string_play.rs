
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word_better(s: &str) -> &str {
    let mut words = s.split_whitespace();
    match words.nth(1) {
        None => &"",
        Some(val) => val
    }
}