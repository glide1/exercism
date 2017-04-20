
fn invalid_char(c: char) -> bool {
    !(c.is_digit(10) || c.is_whitespace())
}

pub fn is_valid(luhn_str: &str) -> bool {

    let filtered_str : Vec<char> = luhn_str.chars().filter(|c| c.is_digit(10) ).collect();

    if luhn_str.chars().any(|c| invalid_char(c)) {
        return false
    }
    if filtered_str.len() < 2 {
        return false
    }
    let values = filtered_str.iter().rev().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let mut sum = 0;
    let mut is_even = false;
    for val in values {
        if is_even {
            let int_val = val * 2;
            sum += if int_val > 9 { int_val - 9 } else { int_val };
        } else {
            sum += val;
        }
        is_even = !is_even;
    }

    sum % 10 == 0
}