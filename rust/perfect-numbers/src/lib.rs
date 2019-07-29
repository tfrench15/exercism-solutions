#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None
    }

    let factors = get_factors(num);
    let aliquot: u64 = factors.iter().sum();

    if aliquot < num {
        return Some(Classification::Deficient)
    } else if aliquot > num {
        return Some(Classification::Abundant)
    } else {
        return Some(Classification::Perfect)
    }

}

fn get_factors(num: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for n in 1..num {
        if num % n == 0 {
            factors.push(n);
        }
    }

    factors
}