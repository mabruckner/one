use std::collections::HashMap;
use std::num::ParseIntError;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct  OneValue { 
    pub data: Vec<u8>
}

impl OneValue {
    pub fn as_string(&self) -> Option<String> {
        match String::from_utf8(self.data.clone()) {
            Ok(value) => Some(value),
            _ => None
        }

    }
    pub fn from_string(s: &str) -> Self {
        OneValue {
            data:String::from(s).into_bytes()
        }
    }
    pub fn parse_hex(s: &str) -> Result<Self, ParseIntError> {
        let mut out = Vec::new();
        let off = if s.len() % 2 == 0 {
            0
        } else {
            1
        };
        for i in 1..(s.len()/2) {
            out.insert(0, u8::from_str_radix(&s[(i*2+off)..(i*2+off+2)], 16)?);
        }
        if off == 1 {
            out.push(u8::from_str_radix(&s[2..3], 16)?);
        }
        Ok(OneValue {
            data: out
        })

    }
}

#[derive(Debug)]
pub enum Token {
    Literal(OneValue),
    Call(OneValue),
}

#[derive(Debug)]
pub struct Block {
    pub items: Vec<Token>
}

#[derive(Debug)]
pub struct OneProgram {
    pub functions: HashMap<OneValue, Block>
}

impl OneProgram {
    pub fn new() -> Self {
        OneProgram {
            functions: HashMap::new()
        }
    }
    pub fn ingest(&mut self, functions: Vec<(OneValue, Block)>) {
        for (val, blk) in functions {
            self.functions.insert(val, blk);
        }
    }
}




