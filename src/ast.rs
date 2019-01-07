use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct  OneValue { 
    pub data: Vec<u8>
}

impl OneValue {
    pub fn from_string(s: &str) -> Self {
        OneValue {
            data:String::from(s).into_bytes()
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Literal(OneValue),
    Call(OneValue),
    Return,
    If
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<Token>
}

#[derive(Debug)]
pub struct OneProgram {
    pub functions: HashMap<OneValue, Block>
}



