pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for num in 1..n+1 {
        sum += num;
    }
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for num in 1..n+1 {
        sum += num.pow(2);
    }
    sum
}

pub fn difference(n: u32) -> u32 {
    let sum_of_sqs = sum_of_squares(n);
    let sq_of_sums = square_of_sum(n);

    sq_of_sums - sum_of_sqs 
}
