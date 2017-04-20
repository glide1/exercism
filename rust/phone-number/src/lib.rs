
pub fn number(number: &str) -> Option<String> {
    let res: Vec<char> = number.chars().filter(|x| x.is_digit(10)).collect();

    match res.len() {
        10 => Some(res.into_iter().collect()),
        11 if res[0] == '1' => Some(res.into_iter().skip(1).collect()),
        _ => None,
    }
}

pub fn area_code(phone_number: &str) -> Option<String> {
    if let Some(x) = number(phone_number) {
        Some(x.chars().take(3).collect())
    } else {
        None
    }
}

pub fn pretty_print(phone_number: &str) -> String {
    if let Some(num) = number(phone_number) {
        format!("({}) {}-{}", &num[0..3], &num[3..6], &num[6..10])
    } else {
        String::from("invalid")
    }
}