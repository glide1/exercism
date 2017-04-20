
#[derive(PartialEq, Debug,Copy,Clone)]
enum BracketTypes {
    OpenBrace,
    OpenSquare,
    OpenParen,
    CloseBrace,
    CloseSquare,
    CloseParen,
}


pub struct Brackets {
    brackets: Vec<BracketTypes>,
}

impl Brackets {
    pub fn from(string: &str) -> Brackets {
        Brackets {
            brackets: string.chars()
                .filter_map(|x| match x {
                                '(' => Some(BracketTypes::OpenParen),
                                ')' => Some(BracketTypes::CloseParen),
                                '[' => Some(BracketTypes::OpenSquare),
                                ']' => Some(BracketTypes::CloseSquare),
                                '{' => Some(BracketTypes::OpenBrace),
                                '}' => Some(BracketTypes::CloseBrace),
                                _ => None,
                            })
                .collect(),
        }
    }

    pub fn are_balanced(self) -> bool {
        let mut stack: Vec<BracketTypes> = Vec::new();

        for current in self.brackets {
            println!("current: {:?} stack {:?}", current, stack);
            match stack.pop() {
                None => stack.push(current),
                Some(val) => {
                    match current {
                        BracketTypes::CloseParen if val == BracketTypes::OpenParen => (),
                        BracketTypes::CloseSquare if val == BracketTypes::OpenSquare => (),
                        BracketTypes::CloseBrace if val == BracketTypes::OpenBrace => (),
                        checking => {
                            stack.push(val);
                            stack.push(checking)
                        }
                    }
                }
            }
        }
        stack.is_empty()
    }
}
