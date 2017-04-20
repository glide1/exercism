use std::collections::BTreeMap;

pub fn anagrams_for<'a>(word: &str, list: &[&'a str]) -> Vec<&'a str> {

    let letter_count = letter_counts(word);

    list.iter()
        .filter_map(|item| if letter_counts(item) == letter_count &&
                              *item.to_lowercase() != word.to_lowercase() {
                        Some(*item)
                    } else {
                        None
                    })
        .collect()
}

fn letter_counts(word: &str) -> BTreeMap<char, usize> {
    word.to_lowercase().chars().fold(BTreeMap::new(), |mut btree, ch| {
        *btree.entry(ch).or_insert(0) += 1;
        btree
    })
}