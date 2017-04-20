
#[derive(Debug)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<T: Eq + Copy> CustomSet<T> {
    pub fn new(items: Vec<T>) -> Self {
        CustomSet { items: items }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn contains(&self, other: &T) -> bool {
        self.items.iter().any(|x| *x == *other)
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.items.iter().all(|x| other.contains(x))
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.items.iter().all(|x| !other.contains(x))
    }

    pub fn add(&mut self, item: T) {
        if !self.contains(&item) {
            self.items.push(item)
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet::new(self.items.iter().fold(Vec::new(), |mut vec, item| {
            if other.contains(item) {
                vec.push(*item)
            }
            vec
        }))
    }

    pub fn difference(&self, other: &Self) -> Self {
        CustomSet::new(self.items.iter().fold(Vec::new(), |mut vec, item| {
            if !other.contains(item) {
                vec.push(*item)
            }
            vec
        }))

    }

    pub fn union(&self, other: &Self) -> Self {
        let mut new_set = CustomSet::new(self.items.clone());

        for x in other.items.clone().into_iter() {
            new_set.add(x)
        }
        new_set
    }
}

impl<T: Eq + Copy> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}