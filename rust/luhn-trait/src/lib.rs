pub trait Luhn {
    fn valid_luhn(&self) -> bool {

        self.luhn_values()
            .iter()
            .enumerate()
            .map(|(i, val)| if i % 2 == 1 {
                     match val * 2 {
                         n if n > 9 => (n - 9) as u32,
                         n => n as u32,
                     }
                 } else {
                     *val as u32
                 })
            .sum::<u32>() % 10 == 0

    }

    fn luhn_values(&self) -> Vec<u8>;
}


impl Luhn for &'static str {
    fn luhn_values(&self) -> Vec<u8> {
        self.chars()
            .filter_map(|x| x.to_digit(10))
            .map(|x| x as u8)
            .collect()
    }
}

impl Luhn for String {
    fn luhn_values(&self) -> Vec<u8> {
        self.chars()
            .filter_map(|x| x.to_digit(10))
            .map(|x| x as u8)
            .collect()
    }
}

macro_rules! implement_luhn_for {
    ($($num:ty), *) => {$(
    impl Luhn for $num {
        fn luhn_values(&self) -> Vec<u8> {
            let mut val = *self;
            let mut ret : Vec<u8> = Vec::new();
            while val > 0 {
                ret.push((val % 10) as u8);
                val /= 10
            }
            ret
        }
    }
    )*}
}

implement_luhn_for!(u8, u16, u32, u64, usize);
