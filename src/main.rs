use std::env::args;
use std::fs;

mod ast;
mod one;
mod logic;

fn main() {
    let mut program = ast::OneProgram::new();
    let mut parser = one::ProgramParser::new();
    program.ingest(parser.parse(include_str!("std.one")).unwrap());
    for arg in args().skip(1) {
        println!("READING FILE {}", arg);
        let data = fs::read_to_string(arg).unwrap();
        program.ingest(parser.parse(&data).unwrap());
    }
    println!("{:?}", program);
    let mut state = logic::State::new();
    loop {
        match state.step(&mut program) {
            Ok(()) => (),
            Err(err) => {
                println!("Program Terminated: {:?}", err);
                break
            }
        }
    }
}
