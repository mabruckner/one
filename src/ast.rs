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
    pub fn as_usize(&self) -> Option<usize> {
        if self.data.len() <= 4 {
            let mut acc:usize = 0;
            for i in 0..self.data.len() {
                acc = acc + ((self.data[i] as usize) << (i*8));
            }
            Some(acc)
        } else {
            None
        }
    }
    pub fn from_usize(u: usize) -> Self {
        OneValue {
            data: vec![
                (u&0xff) as u8,
                ((u>>8)&0xff) as u8,
                ((u>>16)&0xff) as u8,
                ((u>>24)&0xff) as u8]
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
    pub fn add(&self, val: &OneValue) -> OneValue {
        let mut acc: u16 = 0;
        let mut out = Vec::new();
        for i in 0..(self.data.len().max(val.data.len())) {
            let (&a, &b) = (self.data.get(i).unwrap_or(&0), val.data.get(i).unwrap_or(&0));
            acc = acc + a as u16 + b as u16;
            out.push((acc & 0xff) as u8);
            acc = acc >> 8;
        }
        if acc > 0 {
            out.push(acc as u8);
        } if acc < 0 {
            out = vec![0];
        }
        return OneValue {
            data: out
        }
    }
    pub fn sub(&self, val: &OneValue) -> OneValue {
        let mut acc: i16 = 0;
        let mut out = Vec::new();
        for i in 0..(self.data.len().max(val.data.len())) {
            let (&a, &b) = (self.data.get(i).unwrap_or(&0), val.data.get(i).unwrap_or(&0));
            acc = acc + a as i16 - b as i16;
            out.push((acc & 0xff) as u8);
            //println!("{:b}", acc);
            acc = acc >> 8;
        }
        if acc > 0 {
            out.push((acc & 0xff) as u8);
        }
        return OneValue {
            data: out
        }
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




