use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    input.iter().fold(BTreeMap::new(), |mut acc, (val, entries)| {
        for entry in entries {
            acc.insert(entry.to_lowercase(), *val);
        }
        acc
    })
}