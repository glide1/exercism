use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Range;

struct Equation {
    lhs: Member,
    rhs: Member,
}

#[derive(Debug)]
struct Member {
    terms: Vec<Term>,
}

#[derive(Debug, Clone)]
enum Term {
    Operation(Operation),
    Variable(String),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
}

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let equation = Equation::create(puzzle);
    equation.solve()
}

impl Equation {
    fn create(puzzle: &str) -> Self {
        let mut sides = puzzle.split("==");
        Equation {
            lhs: Member::create(sides.next().unwrap()),
            rhs: Member::create(sides.next().unwrap()),
        }
    }

    fn solve(&self) -> Option<HashMap<char, u8>> {
        let mut char_set: HashMap<char, bool> = HashMap::new();
        for lhs_char in self.lhs.characters() {
            let x = char_set.entry(lhs_char.0).or_insert(false);
            *x |= lhs_char.1
        }
        for rhs_char in self.rhs.characters() {
            let x = char_set.entry(rhs_char.0).or_insert(false);
            *x |= rhs_char.1
        }

        let mut value_ranges: HashMap<char, std::ops::Range<_>> = HashMap::new();

        for ch in char_set {
            value_ranges.insert(ch.0, if ch.1 { 1..10 } else { 0..10 });
        }

        let max_length = std::cmp::max(self.lhs.max_term_length(), self.rhs.max_term_length());
        let mut possible_values: Vec<HashMap<char, u8>> = Vec::new();
        let mut solved_characters: HashSet<char> = HashSet::new();
        for equation_size in 1..max_length + 1 {
            let mod_by = 10i32.pow(equation_size as u32);
            let mut characters_needed = self.lhs.characters_up_to_length(equation_size);
            characters_needed.extend(self.rhs.characters_up_to_length(equation_size));

            let partial_value_ranges = characters_needed.iter().fold(HashMap::new(),
                                                                     |mut hm, ch| {
                if solved_characters.insert(*ch) {
                    hm.insert(*ch, value_ranges[ch].clone());
                }
                hm
            });

            println!("partial value ranges: {:?}", partial_value_ranges);
            let current_possible_values = if possible_values.is_empty() {
                create_all_possible_values(partial_value_ranges)
            } else {
                let range_vec = partial_value_ranges.iter().fold(Vec::new(), |mut vec, i| {
                    vec.push((*i.0, (i.1).clone()));
                    vec
                });
                create_possible_values(possible_values, range_vec.as_slice())
            };

            let matched_values: Vec<HashMap<char, u8>> = current_possible_values.iter()
                .filter_map(|values| {
                    let lhs_value = self.lhs.calculate_length(&values, equation_size) % mod_by;
                    let rhs_value = self.rhs.calculate_length(&values, equation_size) % mod_by;
                    if lhs_value == rhs_value {
                        Some(values.clone())
                    } else {
                        None
                    }
                })
                .collect();

            possible_values = matched_values;
            println!("{}: matched_values: {:?}",
                     possible_values.len(),
                     possible_values);

            if possible_values.is_empty() {
                break;
            }
        }

        possible_values.pop()
    }
}


fn create_all_possible_values(value_ranges: HashMap<char, std::ops::Range<u8>>)
                              -> Vec<HashMap<char, u8>> {
    let ranges: Vec<(char, std::ops::Range<u8>)> =
        value_ranges.iter().fold(Vec::new(), |mut vec, (k, v)| {
            vec.push((*k, v.clone()));
            vec
        });

    create_possible_values(Vec::new(), &ranges)
}

fn create_possible_values(to_expand: Vec<HashMap<char, u8>>,
                          ranges: &[(char, Range<u8>)])
                          -> Vec<HashMap<char, u8>> {
    if let Some(x) = ranges.split_first() {
        let ch = (x.0).0;
        let rng = (x.0).clone().1;
        let mut expanded: Vec<HashMap<char, u8>> = Vec::new();
        if !to_expand.is_empty() {
            expanded.extend(to_expand.clone().iter().flat_map(|hash| {
                let fn_hash = hash.clone();

                let mut v = Vec::new();
                for i in rng.clone() {
                    if !fn_hash.values().any(|x| *x == i) {
                        let mut h = fn_hash.clone();
                        h.insert(ch, i);
                        v.push(h);
                    }
                }
                v
            }))
        } else {
            expanded.extend(rng.clone().fold(Vec::new(), |mut v, i| {
                let mut h: HashMap<char, u8> = HashMap::new();
                h.insert(ch, i);
                v.push(h);
                v
            }))
        }
        create_possible_values(expanded, x.1)
    } else {
        to_expand
    }
}


impl Member {
    fn create(member: &str) -> Self {
        let trimmed = member.trim();

        let mut op_stack: Vec<Term> = Vec::new();
        let mut postfix_stack: Vec<Term> = Vec::new();

        for term in trimmed.split(' ') {
            match term {
                "+" => {
                    while {
                              match op_stack.pop() {
                                  Some(o) => {
                            postfix_stack.push(o);
                            true
                        } 
                                  None => false,
                              }
                          } {}
                    op_stack.push(Term::Operation(Operation::Add));
                }
                n => postfix_stack.push(Term::Variable(n.to_uppercase())),
            };
        }

        while {
                  match op_stack.pop() {
                      Some(o) => {
                postfix_stack.push(o);
                true
            }
                      None => false,
                  }
              } {}

        Member { terms: postfix_stack }
    }

    /// Returns the characters that are found on the Member
    /// The tuple is the character and whether or not the character was first
    fn characters(&self) -> Vec<(char, bool)> {
        self.terms
            .clone()
            .into_iter()
            .fold(Vec::new(), |mut vec, item| {
                if let Term::Variable(s) = item {
                    for (i, c) in s.chars().enumerate() {
                        vec.push((c, i == 0))
                    }
                }
                vec
            })
    }

    fn max_term_length(&self) -> usize {
        self.terms
            .clone()
            .into_iter()
            .filter_map(|x| match x {
                            Term::Variable(s) => Some(s.len()),
                            _ => None,
                        })
            .max()
            .unwrap()
    }

    fn calculate_length(&self, values: &HashMap<char, u8>, length: usize) -> i32 {
        let mut value_stack: Vec<i32> = Vec::new();
        for term in self.terms.clone() {
            match term {
                Term::Variable(s) => {
                    value_stack.push(Member::to_integer_len(s.as_ref(), values, length))
                }
                Term::Operation(op) => {
                    let right = value_stack.pop().unwrap();
                    let left = value_stack.pop().unwrap();
                    value_stack.push(op.apply(left, right));
                }
            }
        }

        let x = value_stack.pop();
        x.unwrap()
    }

    fn to_integer_len(s: &str, values: &HashMap<char, u8>, len: usize) -> i32 {
        let val: i32 = s.chars()
            .rev()
            .take(len)
            .enumerate()
            .map(|(i, ch)| 10i32.pow(i as u32) * (*values.get(&ch).unwrap() as i32))
            .sum();
        val
    }

    fn characters_up_to_length(&self, length: usize) -> HashSet<char> {
        self.terms
            .clone()
            .into_iter()
            .filter_map(|item| match item {
                            Term::Variable(s) => Some(s),
                            _ => None,
                        })
            .fold(HashSet::new(), |mut hash, term| {
                for ch in term.chars().rev().take(length) {
                    hash.insert(ch);
                }
                hash
            })
    }
}

impl Operation {
    fn apply<T: std::ops::Add<Output = T>>(&self, left: T, right: T) -> T {
        match *self {
            Operation::Add => left + right,
        }
    }
}
