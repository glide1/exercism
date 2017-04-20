
pub fn convert(input: &str) -> Result<String, ()> {
    let lines: Vec<&str> = input.split("\n").collect();
    if lines.len() % 4 != 0 {
        return Err(());
    }
    if lines[0].len() % 3 != 0 {
        return Err(());
    }

    let mut converted = String::from("");
    let length = lines[0].len();
    let height = lines.len();

    let mut j = 0;

    while j < height {
        let mut i = 0;
        while i < length {
            let top_slice = &lines[j][i..i + 3];
            let mid_slice = &lines[j + 1][i..i + 3];
            let bottom_slice = &lines[j + 2][i..i + 3];
            converted.push(match_character(top_slice, mid_slice, bottom_slice));
            i += 3
        }
        converted.push(',');
        j += 4
    }

    let total_len = converted.len() - 1;
    converted.remove(total_len);

    Ok(converted)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_character(top: &str, mid: &str, bottom: &str) -> char {
    match(top, mid, bottom) {
        (" _ ",
         "| |",
         "|_|") => '0',
        ("   ",
         "  |",
         "  |") => '1',
        (" _ ",
         " _|",
         "|_ ") => '2',
        (" _ ",
         " _|",
         " _|") => '3',
        ("   ",
         "|_|",
         "  |") => '4',
        (" _ ",
         "|_ ",
         " _|") => '5',
        (" _ ",
         "|_ ",
         "|_|") => '6',
        (" _ ",
         "  |",
         "  |") => '7',
        (" _ ",
         "|_|",
         "|_|") => '8',
        (" _ ",
         "|_|",
         " _|") => '9',
        _ => '?',
    }
}

