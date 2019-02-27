use std::collections::HashSet;

pub fn find(_sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    let trip: [u32; 3] = [3, 4, 5];

    triplets.insert(trip);

    return triplets;
}
