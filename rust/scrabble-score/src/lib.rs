#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref CHAR_MAP: HashMap<char, usize> = {
        let mut map = HashMap::new();
        map.insert('a', 1);
        map.insert('b', 3);
        map.insert('c', 3);
        map.insert('d', 2);
        map.insert('e', 1);
        map.insert('f', 4);
        map.insert('g', 2);
        map.insert('h', 4);
        map.insert('i', 1);
        map.insert('j', 8);
        map.insert('k', 5);
        map.insert('l', 1);
        map.insert('m', 3);
        map.insert('n', 1);
        map.insert('o', 1);
        map.insert('p', 3);
        map.insert('q', 10);
        map.insert('r', 1);
        map.insert('s', 1);
        map.insert('t', 1);
        map.insert('u', 1);
        map.insert('v', 4);
        map.insert('w', 4);
        map.insert('x', 8);
        map.insert('y', 4);
        map.insert('z', 10);
        map
    };
}

pub fn score(word: &str) -> usize {
    word.to_lowercase().chars().map( |c| if let Some(&s) = CHAR_MAP.get(&c) { s } else { 0 } ).sum()
}