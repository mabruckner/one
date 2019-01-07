use std::collections::HashMap;
use std::num::ParseIntError;

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



