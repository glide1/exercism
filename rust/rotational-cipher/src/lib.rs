use std::char;

fn a_rotate(ch: &char, rotation: u8) -> char {
    match *ch {
        'A'...'Z' => ((((*ch as u8) - b'A') + rotation) % 26 + b'A') as char,
        'a'...'z' => ((((*ch as u8) - b'a') + rotation) % 26 + b'a') as char,
        c => c,
    }
}

pub fn rotate(input: &str, rotation: u8) -> String {
    input.chars().map(|c| a_rotate(&c, rotation)).collect()
}