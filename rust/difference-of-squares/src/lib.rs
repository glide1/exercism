pub fn square_of_sum(n: u64) -> u64 {
    let mut sum : u64 = n;
    for i in 1..n {
        sum +=i;
    }
    sum * sum
}

pub fn sum_of_squares(n: u64) -> u64 {
    let mut total : u64 = n * n;
    for i in 1..n {
        total += i * i
    }
    total
}

pub fn difference(n: u64) -> u64 {
    square_of_sum(n) - sum_of_squares(n)
}