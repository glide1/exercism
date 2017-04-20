pub struct PascalsTriangle {
    count: u32,
}

fn calc_next_row(current_row: &Vec<u32>) -> Vec<u32> {
    if current_row.len() == 1 {
        vec![1, 1]
    } else {
        let mut ret: Vec<u32> = Vec::new();
        ret.push(1);
        for idx in 0..current_row.len() - 1 {
            ret.push(current_row[idx] + current_row[idx + 1])
        }
        ret.push(1);
        ret
    }
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {

        let mut ret_val: Vec<Vec<u32>> = Vec::new();
        let mut current = vec![1];
        for i in 0..self.count {
            ret_val.push(current.clone());
            current = calc_next_row(&current);
        }
        ret_val
    }
}