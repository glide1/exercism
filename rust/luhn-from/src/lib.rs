pub struct Luhn {
    //vector of u8s in value order
    values: Vec<u8>,
}

impl Luhn {
    pub fn is_valid(self) -> bool {

        if self.values.len() < 2 {
            return false;
        }

        let total: u8 = self.values
            .into_iter()
            .enumerate()
            .map(|(p, v)| if p % 2 == 1 {
                     match v * 2 {
                         n if n > 9 => n - 9,
                         n => n,
                     }
                 } else {
                     v
                 })
            .sum();

        total % 10 == 0
    }
}

macro_rules! implement_from_num {
    ($($num:ty),*) => {$(
        impl From<$num> for Luhn {
            fn from(mut val: $num) -> Luhn {
                let mut values : Vec<u8> = Vec::new();

                while val > 0 {
                    values.push( (val % 10) as u8);
                    val /= 10
                }
                Luhn {
                    values: values
                }
            }
        }
    )*}
}

implement_from_num!(u8, u16, u32, u64, usize);


impl<'a> From<&'a str> for Luhn {
    fn from(val: &str) -> Luhn {

        if val.chars().any(|x| x.is_alphabetic()) {
            return Luhn { values: vec![] };
        }

        Luhn {
            values: val.chars()
                .rev()
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as u8)
                .collect(),
        }
    }
}

impl<'a> From<String> for Luhn {
    fn from(val: String) -> Luhn {
        Luhn::from(val.as_str())
    }
}