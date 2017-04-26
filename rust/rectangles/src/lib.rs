

pub fn count(rectangle: &[&str]) -> usize {
    let height = rectangle.len();
    if let Some(s) = rectangle.get(0) {
        let width = s.len();
        if height < 2 && width < 2 {
            0
        } else {
            let mut found_rectagles = 0;
            let mut current_rectangle = rectangle;
            loop {
                match current_rectangle.split_first() {
                    None => break,
                    Some((line, rest)) => {
                        let mut found_corners = Vec::new();
                        let mut position = 0;
                        let mut pair_corners = Vec::new();
                        for ch in line.chars() {
                            match ch {
                                '+' => {
                                    pair_corners.extend(found_corners.clone().iter().map(|x| (*x, position)));
                                    found_corners.push(position);
                                    }
                                    ,
                                '-' => (),
                                _ => found_corners.clear(),
                            }
                            position += 1;
                        }

                        for corner in pair_corners {
                            let mut current_row = 0;
                            loop {
                                match rest.get(current_row) {
                                    Some(row) => {
                                        let row_bytes = row.as_bytes();
                                        match (row_bytes.get(corner.0), row_bytes.get(corner.1)) {
                                            (Some(&b'+'), Some(&b'+')) => found_rectagles += 1,
                                            (Some(&b'|'), Some(&b'|')) => (),
                                            (Some(&b'+'), Some(&b'|')) => (),
                                            (Some(&b'|'), Some(&b'+')) => (),
                                            (_, _) => break,
                                        }
                                    },
                                    None => break,
                                }
                                current_row += 1;
                            }
                        }
                        current_rectangle = rest;
                    }
                }
            }
            found_rectagles
        }
    } else {
        0
    }
}