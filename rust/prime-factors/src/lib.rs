pub fn factors(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut num = n;
    let mut div = 2;
    
    while num != 1 {
        if num % div == 0 {
            primes.push(div);
            num = num / div;
        } else {
            div += 1;
        }
    }
    primes
}
