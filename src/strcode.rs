
pub fn as_pig_latin(input: &str) -> String {
    let words = input.split_whitespace();
    let pig_latin: Vec<String> = words
        .map(|w: &str| {
            let mut chars = w.chars();
            let first = match chars.next() {
                Some(c) => c,
                None => return String::new(),
            };
            match first {
                'a' | 'e' | 'i' | 'o' | 'u' => { 
                    format!("{}-{}", w, "hay") 
                }
                _ => format!("{}-{}", chars.as_str(), "ay") 
            }
        })
        .collect();
    pig_latin.join(" ")
}