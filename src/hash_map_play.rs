use std::collections::HashMap;

fn hash_map_play() {
    let mut text = "this is this is a long string with some dupes a long long";
    let tokens = text.split_whitespace();
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for token in tokens {
        let entry = counts.entry(token).or_insert(0);
        *entry += 1;
    }
    for (key, value) in &counts {
        println!("{} - {}", key, value);
    }
}