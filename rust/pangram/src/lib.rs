use std::collections::HashMap;

pub fn is_pangram(sentence: &str) -> bool {
    let mut pangram_chars : HashMap<char, bool> = HashMap::new();

    pangram_chars.insert('a', false);
    pangram_chars.insert('b', false);
    pangram_chars.insert('c', false);
    pangram_chars.insert('d', false);
    pangram_chars.insert('e', false);
    pangram_chars.insert('f', false);
    pangram_chars.insert('g', false);
    pangram_chars.insert('h', false);
    pangram_chars.insert('i', false);
    pangram_chars.insert('j', false);
    pangram_chars.insert('k', false);
    pangram_chars.insert('l', false);
    pangram_chars.insert('m', false);
    pangram_chars.insert('n', false);
    pangram_chars.insert('o', false);
    pangram_chars.insert('p', false);
    pangram_chars.insert('q', false);
    pangram_chars.insert('r', false);
    pangram_chars.insert('s', false);
    pangram_chars.insert('t', false);
    pangram_chars.insert('u', false);
    pangram_chars.insert('v', false);
    pangram_chars.insert('w', false);
    pangram_chars.insert('x', false);
    pangram_chars.insert('y', false);
    pangram_chars.insert('z', false);

    for c in sentence.to_lowercase().chars() {
        pangram_chars.insert(c, true);
    }

    pangram_chars.values().all(|val| *val )
}
