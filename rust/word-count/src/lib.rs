use std::collections::HashMap;

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts : HashMap<String, u32> = HashMap::new();
    let fixed_string : String = words.to_lowercase().chars().filter(|c| c.is_whitespace() || c.is_alphanumeric()).collect();
    for word in fixed_string.split_whitespace() {
        *counts.entry(String::from(word)).or_insert(0) += 1
    }

    counts
}