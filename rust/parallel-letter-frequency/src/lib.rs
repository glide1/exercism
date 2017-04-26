extern crate crossbeam;
use std::collections::HashMap;


pub fn frequency(text: &[&str], concur: usize) -> HashMap<char, usize> {

    let mut guards = Vec::with_capacity(concur);
    crossbeam::scope(|scope| {
        let chunk_size = std::cmp::max(text.len() / concur, 1);
        for chunk in text.chunks(chunk_size) {
            guards.push(scope.spawn(move || _frequency(chunk)))
        }
        guards.into_iter().map(|g| g.join()).fold(HashMap::new(), |mut hm, item| {
            for (key, val) in item {
                *hm.entry(key).or_insert(0) += val;
            }
            hm
        })
    })
}

fn _frequency(texts: &[&str]) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for line in texts {
        for chr in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = chr.to_lowercase().next() {
                (*map.entry(c).or_insert(0)) += 1;
            }
        }
    }

    map
}
