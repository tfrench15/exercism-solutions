pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None
    }

    let mut steps = 0;
    let mut num = n;
    while num >= 1 {
        if num == 1 {
            return Some(steps)
        } else if num % 2 == 0 {
            steps += 1;
            num = num / 2;
        } else {
            steps += 1;
            num = 3*num + 1;
        }
    }
    Some(steps)
}

