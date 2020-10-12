mod ast;
mod riscv_parser;

use pest::Parser;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = &args[1];

    let file = std::fs::read_to_string(path).unwrap();

    let pairs = riscv_parser::Parser::parse(riscv_parser::Rule::program, file.as_str())
        .unwrap_or_else(|e| panic!("{}", e));
    let program = ast::Program::from_pest_pairs(pairs);
    for line in program.lines {
        println!("{:?}", line);
    }
}
