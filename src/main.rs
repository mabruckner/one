mod ast;
mod one;

fn main() {
    println!("{:?}", one::FunctionParser::new().parse("HELLO { }"));
    println!("{:?}", one::FunctionParser::new().parse("HELLO { if return }"));
    println!("{:?}", one::FunctionParser::new().parse("HELLO { hello if if return thing98098 }"));
    println!("{:?}", one::FunctionParser::new().parse("hi { hello if if return thing98098 i 0x5ABc983 }"));

}
