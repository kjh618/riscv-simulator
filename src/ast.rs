use crate::riscv_parser;
use phf::phf_map;

#[derive(Debug, Copy, Clone)]
pub struct Register {
    pub index: u8,
}

#[derive(Debug, Clone)]
pub enum JumpTarget {
    Label(String),
    Offset(i32),
}

#[derive(Debug, Copy, Clone)]
pub struct Address {
    pub offset: i32,
    pub base: Register,
}

#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub enum BranchCondition { Eq, Ne, Lt, Ge, Ltu, Geu }

#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub enum LoadWidth { B, H, W, Bu, Hu }

#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub enum StoreWidth { B, H, W }

#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub enum IntegerImmediateOperation { Add, Slt, Sltu, Xor, Or, And, Sll, Srl, Sra }

#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub enum IntegerRegisterOperation { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And }

#[derive(Debug, Clone)]
pub enum Instruction {
    Lui(Register, u32),
    Auipc(Register, u32),
    Jal(Register, JumpTarget),
    Jalr(Register, Register, i32),
    Branch(BranchCondition, Register, Register, JumpTarget),
    Load(LoadWidth, Register, Address),
    Store(StoreWidth, Register, Address),
    IntegerImmediate(IntegerImmediateOperation, Register, Register, i32),
    IntegerRegister(IntegerRegisterOperation, Register, Register, Register),
}

#[derive(Debug, Clone)]
pub struct Label {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Line {
    Instruction(Instruction),
    Label(Label),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub lines: Vec<Line>,
}

fn u32_from_pest_pair(pair: &pest::iterators::Pair<riscv_parser::Rule>) -> u32 {
    match pair.as_rule() {
        riscv_parser::Rule::decimal => u32::from_str_radix(pair.as_str(), 10).unwrap(),
        riscv_parser::Rule::hexadecimal => u32::from_str_radix(&pair.as_str()[2..], 16).unwrap(),
        _ => panic!("Not a number"),
    }
}

fn i32_from_pest_pair(pair: &pest::iterators::Pair<riscv_parser::Rule>) -> i32 {
    match pair.as_rule() {
        riscv_parser::Rule::decimal => i32::from_str_radix(pair.as_str(), 10).unwrap(),
        riscv_parser::Rule::hexadecimal => i32::from_str_radix(&pair.as_str()[2..], 16).unwrap(),
        _ => panic!("Not a number"),
    }
}

impl Register {
    const NAME_TO_INDEX: phf::Map<&'static str, u8> = phf_map! {
        "zero" => 0,
        "ra" => 1,
        "sp" => 2,
        "gp" => 3,
        "tp" => 4,
        "t0" => 5,
        "t1" => 6,
        "t2" => 7,
        "s0" => 8,
        "fp" => 8,
        "s1" => 9,
        "a0" => 10,
        "a1" => 11,
        "a2" => 12,
        "a3" => 13,
        "a4" => 14,
        "a5" => 15,
        "a6" => 16,
        "a7" => 17,
        "s2" => 18,
        "s3" => 19,
        "s4" => 20,
        "s5" => 21,
        "s6" => 22,
        "s7" => 23,
        "s8" => 24,
        "s9" => 25,
        "s10" => 26,
        "s11" => 27,
        "t3" => 28,
        "t4" => 29,
        "t5" => 30,
        "t6" => 31,
    };

    fn from_pest_pair(pair: &pest::iterators::Pair<riscv_parser::Rule>) -> Self {
        assert_eq!(pair.as_rule(), riscv_parser::Rule::register);
        let index = Register::NAME_TO_INDEX[pair.as_str()];
        Self { index }
    }
}

impl JumpTarget {
    fn from_pest_pair(pair: &pest::iterators::Pair<riscv_parser::Rule>) -> Self {
        match pair.as_rule() {
            riscv_parser::Rule::label_name => Self::Label(pair.as_str().to_string()),
            riscv_parser::Rule::number => Self::Offset(i32_from_pest_pair(pair)),
            _ => panic!("Wrong jump target"),
        }
    }
}

impl Address {
    fn from_pest_pair(pair: pest::iterators::Pair<riscv_parser::Rule>) -> Self {
        assert_eq!(pair.as_rule(), riscv_parser::Rule::address);
        let mut pairs = pair.into_inner();
        let offset = i32_from_pest_pair(&pairs.next().unwrap());
        let base = Register::from_pest_pair(&pairs.next().unwrap());
        Address { offset, base }
    }
}

impl BranchCondition {
    fn from_instruction_name(name: &str) -> Self {
        match name {
            "beq" => Self::Eq,
            "bne" => Self::Ne,
            "blt" => Self::Lt,
            "bge" => Self::Ge,
            "bltu" => Self::Ltu,
            "bgeu" => Self::Geu,
            _ => panic!("Not a branch"),
        }
    }
}
impl LoadWidth {
    fn from_instruction_name(name: &str) -> Self {
        match name {
            "lb" => Self::B,
            "lh" => Self::H,
            "lw" => Self::W,
            "lbu" => Self::Bu,
            "lhu" => Self::Hu,
            _ => panic!("Not a load"),
        }
    }
}
impl StoreWidth {
    fn from_instruction_name(name: &str) -> Self {
        match name {
            "sb" => Self::B,
            "sh" => Self::H,
            "sw" => Self::W,
            _ => panic!("Not a store"),
        }
    }
}
impl IntegerImmediateOperation {
    fn from_instruction_name(name: &str) -> Self {
        match name {
            "addi" => Self::Add,
            "slti" => Self::Slt,
            "sltiu" => Self::Sltu,
            "xori" => Self::Xor,
            "ori" => Self::Or,
            "andi" => Self::And,
            "slli" => Self::Sll,
            "srli" => Self::Srl,
            "srai" => Self::Sra,
            _ => panic!("Not an integer immediate instruction"),
        }
    }
}
impl IntegerRegisterOperation {
    fn from_instruction_name(name: &str) -> Self {
        match name {
            "add" => Self::Add,
            "sub" => Self::Sub,
            "sll" => Self::Sll,
            "slt" => Self::Slt,
            "sltu" => Self::Sltu,
            "xor" => Self::Xor,
            "srl" => Self::Srl,
            "sra" => Self::Sra,
            "or" => Self::Or,
            "and" => Self::And,
            _ => panic!("Not an integer register instruction"),
        }
    }
}
impl Instruction {
    fn from_pest_pair(pair: pest::iterators::Pair<riscv_parser::Rule>) -> Self {
        assert_eq!(pair.as_rule(), riscv_parser::Rule::instruction);
        let mut pairs = pair.into_inner();
        let instruction_name = pairs.next().unwrap();
        let argument_list: Vec<_> = pairs.collect();
        match instruction_name.as_str() {
            "lui" => {
                let dest = Register::from_pest_pair(&argument_list[0]);
                let immediate = u32_from_pest_pair(&argument_list[1]);
                Self::Lui(dest, immediate)
            }
            "auipc" => {
                let dest = Register::from_pest_pair(&argument_list[0]);
                let immediate = u32_from_pest_pair(&argument_list[1]);
                Self::Auipc(dest, immediate)
            }
            "jal" => {
                let dest = Register::from_pest_pair(&argument_list[0]);
                let target = JumpTarget::from_pest_pair(&argument_list[1]);
                Self::Jal(dest, target)
            }
            "jalr" => {
                let dest = Register::from_pest_pair(&argument_list[0]);
                let base = Register::from_pest_pair(&argument_list[1]);
                let offset = i32_from_pest_pair(&argument_list[2]);
                Self::Jalr(dest, base, offset)
            }
            "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu" => {
                let condition = BranchCondition::from_instruction_name(instruction_name.as_str());
                let src1 = Register::from_pest_pair(&argument_list[0]);
                let src2 = Register::from_pest_pair(&argument_list[1]);
                let target = JumpTarget::from_pest_pair(&argument_list[2]);
                Self::Branch(condition, src1, src2, target)
            }
            "lb" | "lh" | "lw" | "lbu" | "lhu" => {
                let width = LoadWidth::from_instruction_name(instruction_name.as_str());
                let dest = Register::from_pest_pair(&argument_list[0]);
                let address = Address::from_pest_pair(argument_list[1].clone());
                Self::Load(width, dest, address)
            }
            "sb" | "sh" | "sw" => {
                let width = StoreWidth::from_instruction_name(instruction_name.as_str());
                let src = Register::from_pest_pair(&argument_list[0]);
                let address = Address::from_pest_pair(argument_list[1].clone());
                Self::Store(width, src, address)
            }
            "addi" | "slti" | "sltiu" | "xori" | "ori" | "andi" | "slli" | "srli" | "srai" => {
                let operation =
                    IntegerImmediateOperation::from_instruction_name(instruction_name.as_str());
                let dest = Register::from_pest_pair(&argument_list[0]);
                let src = Register::from_pest_pair(&argument_list[1]);
                let immediate = i32_from_pest_pair(&argument_list[2]);
                Self::IntegerImmediate(operation, dest, src, immediate)
            }
            "add" | "sub" | "sll" | "slt" | "sltu" | "xor" | "srl" | "sra" | "or" | "and" => {
                let operation =
                    IntegerRegisterOperation::from_instruction_name(instruction_name.as_str());
                let dest = Register::from_pest_pair(&argument_list[0]);
                let src1 = Register::from_pest_pair(&argument_list[1]);
                let src2 = Register::from_pest_pair(&argument_list[2]);
                Self::IntegerRegister(operation, dest, src1, src2)
            }
            _ => panic!("Unsupported instruction: {}", instruction_name.as_str()),
        }
    }
}

impl Label {
    fn from_pest_pair(pair: pest::iterators::Pair<riscv_parser::Rule>) -> Self {
        assert_eq!(pair.as_rule(), riscv_parser::Rule::label);
        let label_name = pair.into_inner().next().unwrap();
        Self {
            name: label_name.as_str().to_string(),
        }
    }
}

impl Program {
    pub fn from_pest_pairs(pairs: pest::iterators::Pairs<riscv_parser::Rule>) -> Self {
        let mut lines = Vec::new();
        for pair in pairs {
            let line = match pair.as_rule() {
                riscv_parser::Rule::instruction => {
                    Line::Instruction(Instruction::from_pest_pair(pair))
                }
                riscv_parser::Rule::label => Line::Label(Label::from_pest_pair(pair)),
                _ => panic!("Wrong line"),
            };
            lines.push(line);
        }
        Self { lines }
    }
}
