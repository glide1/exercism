use std::collections::HashMap;
use std::collections::hash_map::Entry;

static VALID_CHARS: &'static [char] = &['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, ()> {
    if !VALID_CHARS.into_iter().any(|&c| nucleotide == c) {
        Err(())
    } else if dna.chars().any(|n| !VALID_CHARS.into_iter().any(|&c| n == c )) {
        Err(())
    } else {
        Ok(dna.chars().filter(|&c| c == nucleotide).count())
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, ()> {
    let mut ret_hashmap : HashMap<char, usize> = HashMap::new();
    ret_hashmap.insert('A', 0);
    ret_hashmap.insert('C', 0);
    ret_hashmap.insert('G', 0);
    ret_hashmap.insert('T', 0);

    for n in dna.chars() {
        match ret_hashmap.entry(n) {
            Entry::Occupied(mut o) => *o.get_mut() += 1,
            Entry::Vacant(_) => return Err(()),
        };
    }

    Ok(ret_hashmap)
}