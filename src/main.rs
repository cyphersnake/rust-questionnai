#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
enum ByteCode {
    LoadVal(i32),
    WriteVar(String),
    ReadVar(String),
    Add,
    Multiply,
    ReturnValue,
}

struct Interpreter {
    stack: Vec<i32>,
    vars: HashMap<String, i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Error {
    LoadFromEmptyStack,
    UnknownVarName,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            stack: Vec::new(),
            vars: HashMap::new(),
        }
    }

    fn execute(&mut self, bytecode: &[ByteCode]) -> Result<Option<i32>, Error> {
        for code in bytecode {
            match code {
                ByteCode::LoadVal(val) => self.stack.push(*val),
                ByteCode::WriteVar(varname) => {
                    let value = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                    self.vars.insert(varname.to_string(), value);
                }
                ByteCode::ReadVar(var) => {
                    let valname = *self.vars.get(var).ok_or(Error::UnknownVarName)?;
                    self.stack.push(valname);
                }
                ByteCode::Add => {
                    let a = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                    let b = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                    self.stack.push(a + b);
                }
                ByteCode::Multiply => {
                    let a = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                    let b = self.stack.pop().ok_or(Error::LoadFromEmptyStack)?;
                    self.stack.push(a * b);
                }
                ByteCode::ReturnValue => {
                    return self.stack.pop().map(Some).ok_or(Error::LoadFromEmptyStack);
                }
            }
            pc += 1;
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::{ByteCode, Interpreter};

    #[test]
    fn from_task() {
        assert_eq!(
            Interpreter::new().execute(&[
                ByteCode::LoadVal(1),
                ByteCode::WriteVar("x"),
                ByteCode::LoadVal(2),
                ByteCode::WriteVar("y"),
                ByteCode::ReadVar("x"),
                ByteCode::LoadVal(1),
                ByteCode::Add,
                ByteCode::ReadVar("y"),
                ByteCode::Multiply,
                ByteCode::ReturnValue,
            ]),
            Ok(Some(4))
        );
    }
}

fn main() {
    println!(
        "Result: {:?}",
        Interpreter::new().execute(&[
            ByteCode::LoadVal(1),
            ByteCode::WriteVar("x"),
            ByteCode::LoadVal(2),
            ByteCode::WriteVar("y"),
            ByteCode::ReadVar("x"),
            ByteCode::LoadVal(1),
            ByteCode::Add,
            ByteCode::ReadVar("y"),
            ByteCode::Multiply,
            ByteCode::ReturnValue,
        ])
    );
}
