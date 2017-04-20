use std::ascii::AsciiExt;

fn crypt(input: char) -> char {
    match input {
        'a' => 'z',
        'b' => 'y',
        'c' => 'x',
        'd' => 'w',
        'e' => 'v',
        'f' => 'u',
        'g' => 't',
        'h' => 's',
        'i' => 'r',
        'j' => 'q',
        'k' => 'p',
        'l' => 'o',
        'm' => 'n',
        'n' => 'm',
        'o' => 'l',
        'p' => 'k',
        'q' => 'j',
        'r' => 'i',
        's' => 'h',
        't' => 'g',
        'u' => 'f',
        'v' => 'e',
        'w' => 'd',
        'x' => 'c',
        'y' => 'b',
        'z' => 'a',
        n => n
    }
}


pub fn encode(input: &str) -> String {
    let sized_input : Vec<_> = input.to_lowercase().chars().filter(|c| c.is_alphanumeric() && c.is_ascii()).map(|c| crypt(c)).collect();
    let chunks : Vec<_> = sized_input.chunks(5).map(|ch| (*ch).iter().cloned().collect::<String>() ).collect();
    chunks.join(" ")
}

pub fn decode(input: &str) -> String {
    let sized_input : String = input.to_lowercase().chars().filter(|c| c.is_alphanumeric() && c.is_ascii()).map(|c| crypt(c)).collect();
    sized_input
}