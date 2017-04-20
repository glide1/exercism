
pub fn hamming_distance(a: &str, b: &str) -> Result<u32, ()> {
    let mut a_chars = a.chars();
    let mut b_chars = b.chars();

    let mut a_option = a_chars.next();
    let mut b_option = b_chars.next();

    let mut dist = 0;

    if a.len() != b.len() {
        return Err(())
    }

    while a_option.is_some() && b_option.is_some() {

        if a_option.unwrap() != b_option.unwrap() {
            dist += 1;
        }

        a_option = a_chars.next();
        b_option = b_chars.next();
    }
    return Ok(dist)
}