type Domino = (usize, usize);

pub fn chain(dominoes: &[Domino]) -> Option<Vec<Domino>> {
    let mut start: Option<usize> = None;
    let mut end: usize = 0;

    let mut dominoes = Vec::from(dominoes);
    let mut solution: Vec<Domino> = Vec::new();

    let mut possibilities: Vec<(Vec<Domino>, Vec<Domino>)> = Vec::new();

    if let Some((x, y)) = dominoes.pop() {
        start = Some(x);
        end = y;
        solution.push((x, y));
    }
    loop {
        possibilities.extend::<Vec<(Vec<Domino>, Vec<Domino>)>>(dominoes
                                 .iter()
                                 .filter_map(|&(head, tail)| if head == end || tail == end {
                                                 let mut solution_clone = solution.clone();
                                                 let mut dominoes_clone = dominoes.clone();
                                                 let pos = dominoes_clone
                                                     .iter()
                                                     .position(|&(h, t)| {
                                                                   h == head && tail == t
                                                               })
                                                     .unwrap();
                                                 dominoes_clone.remove(pos);
                                                 if tail == end {
                                                     solution_clone.push((tail, head));
                                                 } else {
                                                     solution_clone.push((head, tail));
                                                 }
                                                 Some((solution_clone, dominoes_clone))
                                             } else {
                                                 None
                                             })
                                 .collect());

        match possibilities.pop() {
            Some((sol, domi)) => {
                solution = sol;
                dominoes = domi;
                end = solution[solution.len() - 1].1;
                println!("Possible: {:?}. End {:?}. Dominoes {:?}.",
                         solution,
                         end,
                         dominoes);
                if dominoes.is_empty() {
                    break;
                }
            }
            None => break,
        }
    }

    if start.unwrap_or(0) == end && dominoes.is_empty() {
        Some(solution)
    } else {
        None
    }
}

