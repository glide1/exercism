pub fn verse(number: i32) -> String {
    match number {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => {
            format!("{0} bottle of beer on the wall, {0} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
                    number)
        }
        x => {
            let remaining = x - 1;
            let bottle_string = match remaining {
                1 => "bottle",
                _ => "bottles",
            };
            format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} {2} of beer on the wall.\n",
                    x,
                    remaining,
                    bottle_string)
        }
    }
}

pub fn sing(from: i32, to: i32) -> String {
    let mut vec = Vec::<String>::new();
    for i in (to..from+1).rev() {
        vec.push(verse(i));
    }
    return vec.as_slice().join("\n")
}