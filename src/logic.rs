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
    StackUnderflow,
    ValueTooLarge
}

#[derive(Debug)]
pub enum ProgramError {
    Finished
}

impl State {
    pub fn new() -> Self {
        State {
            fstack: vec![(OneValue::from_string("main"), 0)],
            pstack: Vec::new(),
            skip: 0
        }
    }
    pub fn step(&mut self, program: &mut OneProgram) -> Result<(), ProgramError> {
        if let Some((fname, pos)) = self.fstack.last().cloned() {
            let block = program.functions.get(&fname).unwrap(); // fix this... eventually
            if pos >= block.items.len() {
                self.fstack.pop();
                return Ok(())
            }
            if self.skip == 0 {
                self.fstack.last_mut().map(|l| l.1 += 1);
                match block.items[pos] {
                    Token::Literal(ref value) => self.pstack.push(value.clone()),
                    Token::Call(ref value) => match self.builtin(value) {
                        Some(Ok(())) => (),
                        Some(Err(err)) => panic!("{:?}", err),
                        None => {
                            if pos == block.items.len() {
                                self.fstack.pop().unwrap();
                            }
                            self.fstack.push((value.clone(), 0));
                        }
                    }
                }
            } else {
                self.fstack.last_mut().map(|l| l.1 += 1);
                self.skip -= 1;
            }
            Ok(())
        } else {
            Err(ProgramError::Finished)
        }
    }
    fn builtin(&mut self, name: &OneValue) -> Option<Result<(), BuiltinError>> {
        let strname = name.as_string();
        Some(match strname.as_ref().map(|n| n.as_str()) {
            Some(val) => match val {
                "print" => self.print(),
                "copy" => {
                    match self.pop() {
                        Ok(val) => {
                        self.pstack.push(val.clone());
                    self.pstack.push(val);
                    Ok(())
                        },
                        Err(x) => Err(x)
                    }
                },
                "drop" => {
                    match self.pop() {
                        Ok(_) => Ok(()),
                        Err(x) => Err(x)
                    }
                },
                "add" => self.add(),
                "sub" => self.sub(),
                "skip" => self.skip(),
                "nonzerop" => self.nonzerop(),
                _ => return None
            },
            _ => return None
        })
    }
    fn skip(&mut self) -> Result<(), BuiltinError> {
        if let Some(num) = self.pop()?.as_usize() {
            self.skip = num;
            Ok(())
        } else {
            Err(BuiltinError::ValueTooLarge)
        }
    }
    fn nonzerop(&mut self) -> Result<(), BuiltinError> {
        if(self.pop()? == OneValue{data:vec![0]}) {
            self.skip = 1;
        }
        Ok(())
    }
    fn add(&mut self) -> Result<(), BuiltinError> {
        self.pop()?.add(&self.pop()?);
        Ok(())
    }
    fn sub(&mut self) -> Result<(), BuiltinError> {
        let rh = self.pop()?;
        let res = self.pop()?.sub(&rh);
        self.pstack.push(res);
        Ok(())
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
