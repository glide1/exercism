pub fn sum_of_multiples(num: u32, multiples: &[u32]) -> u32 {
    (1..num).filter(|&i| multiples.into_iter().any(|x| i % x == 0)).sum()
}