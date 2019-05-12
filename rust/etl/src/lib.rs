use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_system = BTreeMap::new();

    for (key, value) in h.iter() {
        for ch in value {
            new_system.entry(ch.to_ascii_lowercase()).or_insert(*key);
        }
    }
    new_system
}
