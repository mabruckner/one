use ast::*;
use std::io::{self, Write};

pub struct State {
    pub fstack: Vec<(OneValue, usize)>,
    pub pstack: Vec<OneValue>,
    pub skip: usize
}

#[derive(Debug)]
pub enum BuiltinError {
    FileNotFound,
    StackUnderflow
}

impl State {
    pub fn new() -> Self {
        State {
            fstack: vec![(OneValue::from_string("main"), 0)],
            pstack: Vec::new(),
            skip: 0
        }
    }
    pub fn step(&mut self, program: &mut OneProgram) {
        if let Some((fname, pos)) = self.fstack.last().cloned() {
            let block = program.functions.get(&fname).unwrap(); // fix this... eventually
            if pos >= block.items.len() {
                self.fstack.pop();
                return
            }
            if self.skip == 0 {
                self.fstack.last_mut().map(|l| l.1 += 1);
                match block.items[pos] {
                    Token::Literal(ref value) => self.pstack.push(value.clone()),
                    Token::Call(ref value) => match self.builtin(value) {
                        Some(Ok(())) => (),
                        Some(Err(err)) => panic!("{:?}", err),
                        None => unimplemented!()
                    }
                }
            } else {
                self.fstack.last_mut().map(|l| l.1 += 1);
                self.skip -= 1;
            }
        }
    }
    fn builtin(&mut self, name: &OneValue) -> Option<Result<(), BuiltinError>> {
        let strname = name.as_string();
        Some(match strname.as_ref().map(|n| n.as_str()) {
            Some(val) => match val {
                "print" => self.print(),
                _ => return None
            },
            _ => return None
        })
    }
    fn pop(&mut self) -> Result<OneValue, BuiltinError> {
        match self.pstack.pop() {
            Some(val) => Ok(val),
            None => Err(BuiltinError::StackUnderflow)
        }
    }
    fn print(&mut self) -> Result<(), BuiltinError> {
        io::stdout().write(&(self.pop()?.data));
        Ok(())
    }

}
