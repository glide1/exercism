#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: Eq + Ord + Copy>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        return Comparison::Equal
    }

    if a.is_empty() {
        return Comparison::Sublist
    }
    if b.is_empty() {
        return Comparison::Superlist
    }

    if b.windows(a.len()).any(|b_window| b_window == a) {
        Comparison::Sublist
    } else if a.windows(b.len()).any(|a_window| a_window == b) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}