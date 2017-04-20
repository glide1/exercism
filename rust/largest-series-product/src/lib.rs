pub fn lsp(digits: &str, length: usize) -> Result<u32, ()> {
    if length > digits.len() {
        return Err(())
    } 

    let opt_values: Vec<Option<u32>> = digits.chars().map(|c| c.to_digit(10) ).collect();

    if opt_values.iter().any(|val| val.is_none()) {
        return Err(())
    }

    let values : Vec<u32> = opt_values.into_iter().map(|val| val.unwrap()).collect();
    let mut found_lsp = 0;

    let loop_len = values.len();
    for idx in 0.. loop_len - length + 1 {
        let this_lsp = values.iter().skip(idx).take(length).fold(1u32, |tot, val| tot * val);
        if this_lsp > found_lsp {
            found_lsp = this_lsp
        }
    }

    Ok(found_lsp) 
}