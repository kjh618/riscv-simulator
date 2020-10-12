mod riscv_parser;
mod instruction;

use pest::Parser;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = &args[1];

    let program = std::fs::read_to_string(path).unwrap();
    let pairs = riscv_parser::Parser::parse(riscv_parser::Rule::program, program.as_str()).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        println!("[{:?}] {}", pair.as_rule(), pair.as_str());
        instruction::Instruction::from_pest_pair(pair);
    }
}
