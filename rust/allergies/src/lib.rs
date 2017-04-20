#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Allergen {
    Peanuts,
    Cats,
    Strawberries,
    Eggs,
    Shellfish,
    Tomatoes,
    Chocolate,
    Pollen,
}

use Allergen::*;


pub struct Allergies {
    num: u32,
}

impl Allergies {
    pub fn new(num: u32) -> Self {
        Allergies { num: num }
    }


    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        if let Some(s) = allergen.value() {
            s & self.num > 0
        } else {
            false
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::all_values()
            .iter()
            .filter_map(|&x| if let Some(val) = x.value() {
                            if val & self.num > 0 { Some(x) } else { None }
                        } else {
                            None
                        })
            .collect()
    }
}

impl Allergen {
    pub fn value(&self) -> Option<u32> {
        match *self {
            Eggs => Some(1),
            Peanuts => Some(2),
            Shellfish => Some(4),
            Strawberries => Some(8),
            Tomatoes => Some(16),
            Chocolate => Some(32),
            Pollen => Some(64),
            Cats => Some(128),
        }
    }
    pub fn all_values() -> Vec<Allergen> {
        vec![Peanuts, Cats, Strawberries, Eggs, Shellfish, Tomatoes, Chocolate, Pollen]
    }
}