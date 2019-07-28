use std::collections::HashMap;

fn generate_score_table() -> HashMap<char, u64> {
    [('a', 1),
     ('b', 3),
     ('c', 3),
     ('d', 2),
     ('e', 1),
     ('f', 4),
     ('g', 2),
     ('h', 4),
     ('i', 1),
     ('j', 8),
     ('k', 5),
     ('l', 1),
     ('m', 3),
     ('n', 1),
     ('o', 1),
     ('p', 3),
     ('q', 10),
     ('r', 1),
     ('s', 1),
     ('t', 1),
     ('u', 1),
     ('v', 4),
     ('w', 4),
     ('x', 8),
     ('y', 4),
     ('z', 10)]
     .iter().cloned().collect()
}
/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let score_table = generate_score_table();
    word.to_lowercase().chars()
                       .map(|ch| {
                           match score_table.get(&ch) {
                               None => { 0 as u64 },
                               Some(v) => { *v },
                           }
                       })
                       .sum()
}


