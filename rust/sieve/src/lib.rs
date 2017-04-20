use std::ops::Add;

fn step_by<T, F>(start: T, end_exclusive: T, step: T, mut body: F)
    where T: Add<Output = T> + PartialOrd + Copy,
          F: FnMut(T)
{
    let mut i = start;
    while i < end_exclusive {
        body(i);
        i = i + step;
    }
}

pub fn primes_up_to(num: usize) -> Vec<usize> {
    let mut arry = vec![true; num as usize + 1];
    let mut ret_vec: Vec<usize> = vec![];

    for n in 2..(num + 1) {
        if arry[n] {
            ret_vec.push(n);
            step_by(n, num+1, n, |m| {
                arry[m] = false
            });
        }
    }

    ret_vec
}
