use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "riscv.pest"]
struct RiscVParser;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = &args[1];

    let program = std::fs::read_to_string(path).unwrap();
    let pairs = RiscVParser::parse(Rule::program, program.as_str()).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        println!("[{:?}] {}", pair.as_rule(), pair.as_str());
    }
}
