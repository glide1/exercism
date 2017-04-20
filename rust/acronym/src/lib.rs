pub fn abbreviate(input: &str) -> String {

    let mut abbr = String::new();

    let mut prior_ch = ' ';

    for ch in input.chars() {
        if ch.is_alphanumeric() &&
           (!prior_ch.is_alphanumeric() || (prior_ch.is_lowercase() && ch.is_uppercase())) {
            abbr.push(ch)
        }
        prior_ch = ch
    }

    abbr.to_uppercase()
}