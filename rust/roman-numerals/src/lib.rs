pub struct Roman {
    value: isize,
}

impl Roman {
    pub fn from(value: isize) -> Roman {
        Roman { value: value }
    }

    fn highest_roman_value(val: isize) -> (isize, &'static str) {
        match val {
            1...3 => (1, "I"),
            4 => (4, "IV"),
            5...8 => (5, "V"),
            9 => (9, "IX"),
            10...39 => (10, "X"),
            40...49 => (40, "XL"),
            50...99 => (50, "L"),
            100...399 => (100, "C"),
            400...499 => (400, "CD"),
            500...899 => (500, "D"),
            900...999 => (900, "CM"),
            _ => (1000, "M")
        }
    }

    pub fn to_string(&self) -> String {
        let mut rm = String::new();
        let mut current_val = self.value;

        while current_val > 0 {
            let val = Roman::highest_roman_value(current_val);
            rm.push_str(val.1);
            current_val -= val.0
        }
        rm
    }
}