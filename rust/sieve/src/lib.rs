pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound <= 1 {
        return Vec::new();
    }

    let mut primes: Vec<u64> = (2..upper_bound+1).collect();
    println!("{:?}", primes);

    primes.retain(|n| is_prime(*n));

    primes

}

pub fn is_prime(num: u64) -> bool {
    let mut i = 2;
    while i < num {
        if num % i == 0 {
            return false
        }
        i += 1;
    }
    
    true
}