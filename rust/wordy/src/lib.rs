extern crate regex;

#[derive(Debug)]
pub struct WordProblem {
    statements: Vec<Statement>,
}

#[derive(Debug, Clone, Copy)]
enum Statement {
    Value(i32),
    Op(Operation),
    Unsupported,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Minus,
    Multiply,
    Divide,
}

impl WordProblem {
    pub fn new(command: &str) -> WordProblem {
        let re = regex::Regex::new(r" |\?").unwrap();
        let possible_commands = re.split(command);
        let statements: Vec<Statement> =
            possible_commands.filter_map(|word| match word {
                                             "plus" => Some(Statement::Op(Operation::Add)),
                                             "minus" => Some(Statement::Op(Operation::Minus)),
                                             "multiplied" => {
                                                 Some(Statement::Op(Operation::Multiply))
                                             }
                                             "divided" => Some(Statement::Op(Operation::Divide)),
                                             "Who" => Some(Statement::Unsupported),
                                             "cubed" => Some(Statement::Unsupported),
                                             n => {
                                                 if let Ok(value) = n.parse::<i32>() {
                                                     Some(Statement::Value(value))
                                                 } else {
                                                     None
                                                 }
                                             }
                                         })
                .collect();
        WordProblem { statements: statements }
    }

    pub fn answer(&self) -> Result<i32, ()> {

        let mut op_stack: Vec<Operation> = Vec::new();
        let mut postfix_stack: Vec<Statement> = Vec::new();

        for statement in self.statements.clone() {
            match statement {
                Statement::Value(n) => postfix_stack.push(Statement::Value(n)),
                Statement::Op(n) => {

                    while {
                              let x = op_stack.pop();
                              match x {
                                  Some(o) => {
                            postfix_stack.push(Statement::Op(o));
                            true
                        }
                                  None => false,
                              }
                          } {}
                    op_stack.push(n);
                }
                Statement::Unsupported => return Err(()),
            }
        }
        while {
                  let x = op_stack.pop();
                  match x {
                      Some(o) => {
                postfix_stack.push(Statement::Op(o));
                true
            }
                      None => false,
                  }
              } {}

        println!("{:?}", postfix_stack);

        let mut value_stack: Vec<i32> = Vec::new();
        for e in postfix_stack {
            match e {
                Statement::Value(n) => value_stack.push(n),
                Statement::Op(op) => {
                    let right = value_stack.pop().unwrap();
                    let left = value_stack.pop().unwrap();
                    value_stack.push(op.apply(left, right))
                }
                _ => (),
            }
        }

        if let Some(value) = value_stack.pop() {
            Ok(value)
        } else {
            Err(())
        }
    }
}


impl Operation {
    fn apply(&self, left: i32, right: i32) -> i32 {
        match *self {
            Operation::Add => left + right,
            Operation::Minus => left - right,
            Operation::Multiply => left * right,
            Operation::Divide => left / right,
        }
    }
}