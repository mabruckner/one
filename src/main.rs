mod ast;
mod one;
mod logic;

fn main() {
    let mut program = ast::OneProgram::new();
    program.ingest(one::ProgramParser::new().parse("main { 0x0a210a print }").unwrap());
    println!("{:?}", program);
    let mut state = logic::State::new();
    state.step(&mut program);
    state.step(&mut program);
    state.step(&mut program);

}
