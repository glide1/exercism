use std::collections::HashMap;

pub struct Triangle {
    sides_count : HashMap<usize, usize>
}

impl Triangle {
    pub fn build(sides: [usize; 3]) -> Result<Triangle, ()> {
        if sides.into_iter().any(|&x| x == 0) {
            return Err(());
        }

        let max_size : usize = sides.into_iter().map(|&x| x).max();
        let sum : usize = sides.into_iter().sum();
        if (sum - max_size) < max_size {
            return Err(())
        }
        

        Ok(Triangle { sides_count: sides.iter().fold(HashMap::new(), |mut map, size| {
            *map.entry(*size).or_insert(0) += 1;
            map
        }) })

    }

    pub fn is_equilateral(&self) -> bool {
        self.sides_count.values().count() == 1
    }

    pub fn is_scalene(&self) -> bool {
        self.sides_count.values().count() == 3
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides_count.values().count() == 2
    }
}