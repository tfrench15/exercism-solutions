/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    match s1.len() == s2.len() {
        false => { None },
        true => { 
            let mut distance = 0;
            for pair in s1.chars().zip(s2.chars()) {
                if pair.0 != pair.1 {
                    distance += 1;
                }
            }

            Some(distance)
        }
    }
}
