pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for num in 1..limit {
        if is_divisible(num, factors) {
            sum += num
        }
    }
    sum
}

fn is_divisible(num: u32, factors: &[u32]) -> bool {
    for factor in factors {
        if *factor == 0 {
            continue;
        }
        if num % factor == 0 {
            return true
        }
    }
    false
}
