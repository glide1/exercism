use std::collections::BTreeMap;

pub struct School {
    student_grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            student_grades: BTreeMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.student_grades.entry(grade).or_insert(Vec::<String>::new());
        students.push(String::from(student))
    }

    pub fn grades(&self) -> Vec<u32> {
        self.student_grades.keys().map(|&g| g).collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.student_grades.get(&grade).cloned()
    }
}
