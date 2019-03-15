pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2
    }

    let mut primes = vec![2];
    let mut number = 3;
    let mut counter = 0;
    while counter < n {
        if is_prime(number, &primes) == true {
            &primes.push(number);
            number += 1;
            counter += 1;
        } else {
            number += 1;
        }
    }
    let idx = primes.len();
    primes[idx-1]
}

fn is_prime(n: u32, primes: &Vec<u32>) -> bool {
    for prime in primes {
        if n % prime == 0 {
            return false
        }
    }
    true
}

