extern crate regex;

use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Debug)]
pub struct Forth {
    stack: Vec<Value>,
    replacements: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

fn split_input(ch: char) -> bool {
    return ch.is_whitespace() || ch == '\u{0000}' || ch == '\u{0001}';
}

impl Forth {
    pub fn new() -> Self {
        Forth {
            stack: Vec::new(),
            replacements: HashMap::new(),
        }
    }

    pub fn format_stack(&self) -> String {
        self.stack
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {

        let mut start_place = 0;
        let lowercased = input.to_lowercase();
        let mut current_input : &str;

        loop {
            current_input = &lowercased[start_place .. ];
            println!("current_input: {}", current_input);
            if let Some(statement_location) = current_input.find(';') {
                if let Err(e) = self.eval_line(current_input) {
                    return Err(e)
                }
                start_place = statement_location + 1
            } else {
                return self.eval_line(current_input)
            }
        }
    }

    fn eval_line(&mut self, input: &str) -> ForthResult {

        println!("eval_line {:?}", self);
        if input.trim().starts_with(':') {
            return self.add_replacement(input);
        } else {

            let replaced_statements: Vec<String> = input
                .to_lowercase()
                .split(split_input)
                .map(|statement| if let Some(replace) = self.replacements.get(statement) {
                        println!("trying to replace: {:?} with {:?}", statement, replace);
                         replace.clone()
                     } else {
                         String::from(statement)
                     })
                .collect();

            for i in replaced_statements {
                println!("i: {:?}", i);
                match self.eval_statement(i.as_str()) {
                    Err(e) => return Err(e),
                    _ => (),
                }
            }
            Ok(())
        }
    }

    fn eval_statement(&mut self, input: &str) -> ForthResult {
        for i in input.to_lowercase().split(split_input) {
            match i {
                "+" => {
                    match self.operand(|lhs, rhs| lhs + rhs) {
                        Ok(result) => self.stack.push(result),
                        Err(error) => return Err(error),
                    }
                }
                "-" => {
                    match self.operand(|lhs, rhs| lhs - rhs) {
                        Ok(result) => self.stack.push(result),
                        Err(error) => return Err(error),
                    }
                }
                "*" => {
                    match self.operand(|lhs, rhs| lhs * rhs) {
                        Ok(result) => self.stack.push(result),
                        Err(error) => return Err(error),
                    }
                }
                "/" => {
                    match self.divide() {
                        Ok(result) => self.stack.push(result), 
                        Err(error) => return Err(error),
                    }
                }
                "dup" => {
                    match self.stack.pop() {
                        Some(val) => {
                            self.stack.push(val);
                            self.stack.push(val);
                        }
                        None => return Err(Error::StackUnderflow),
                    }
                }
                "drop" => {
                    match self.stack.pop() {
                        None => return Err(Error::StackUnderflow),
                        _ => (),
                    }
                }
                "swap" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(a), Some(b)) => {
                            self.stack.push(a);
                            self.stack.push(b);
                        }
                        _ => return Err(Error::StackUnderflow),
                    }
                }
                "over" => {
                    match (self.stack.pop(), self.stack.pop()) {
                        (Some(a), Some(b)) => {
                            self.stack.push(b);
                            self.stack.push(a);
                            self.stack.push(b);
                        }
                        _ => return Err(Error::StackUnderflow),
                    }
                }
                other => {
                    println!("other: {}", other);
                    if let Ok(num) = other.parse::<i32>() {
                        print!("inserting: {}", num);
                        self.stack.push(num);
                    } else {
                        if other != "" {
                            return Err(Error::UnknownWord)
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn add_replacement(&mut self, input: &str) -> ForthResult {
        println!("add_replacement: {}", input);

        match (input.find(":"), input.find(";")) {
            (Some(start), Some(end)) => {
            let mut replacment_strings = input[start + 1 .. end].trim().splitn(2, split_input);

            let a = replacment_strings.next();
            let b = replacment_strings.next();
            match (a, b) {
            (Some(a), Some(b)) => {
                if let Ok(_) = a.parse::<Value>() {
                    return Err(Error::InvalidWord);
                }
                self.replacements
                    .insert(String::from(a), String::from(b));
            }
            (_, _) => (),
            }
            }
            _ => return Err(Error::InvalidWord)
        }
        Ok(())        
    }

    fn operand<F>(&mut self, op: F) -> Result<Value, Error>
        where F: Fn(Value, Value) -> Value
    {
        let right = self.stack.pop();
        let left = self.stack.pop();
        match (left, right) {
            (Some(lhs), Some(rhs)) => Ok(op(lhs, rhs)),
            _ => Err(Error::StackUnderflow),
        }
    }

    fn divide(&mut self) -> Result<Value, Error> {
        let right = self.stack.pop();
        let left = self.stack.pop();
        match (left, right) {
            (_, Some(0)) => Err(Error::DivisionByZero),
            (Some(lhs), Some(rhs)) => Ok(lhs / rhs),
            _ => Err(Error::StackUnderflow),
        }
    }
}
