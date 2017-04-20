pub fn reply(input: &str) -> &str {
    let uppercase = input.to_uppercase();
    if input.is_empty() {
        return "Fine. Be that way!";
    } else if uppercase == input {
        return "Whoa, chill out!";
    } else if input.ends_with("?") {
        return "Sure.";
    }
    "Whatever."
}