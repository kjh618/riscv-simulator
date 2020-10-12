use phf::phf_map;
use crate::riscv_parser;

const REGISTER_NAME_TO_INDEX: phf::Map<&str, u8> = phf_map! {
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

pub struct Register {
    index: u8,
}

impl Register {
    fn from_pest_pair(pair: &pest::iterators::Pair<riscv_parser::Rule>) -> Self {
        assert_eq!(pair.as_rule(), riscv_parser::Rule::register);
        let index = REGISTER_NAME_TO_INDEX[pair.as_str()];
        Self { index }
    }
}

pub struct Address {
    offset: i32,
    base: Register,
}

pub enum BranchCondition { Eq, Ne, Lt, Ge, Ltu, Geu }
pub enum LoadWidth { B, H, W, Bu, Hu }
pub enum StoreWidth { B, H, W }
pub enum IntegerImmediateOperation { Add, Slt, Sltu, Xor, Or, And, Sll, Srl, Sra }
pub enum IntegerRegisterOperation { Add, Sub, Sll, Slt, Sltu, Xor, Srl, Sra, Or, And }

pub enum Instruction {
    Lui(Register, u32),
    Auipc(Register, u32),
    Jal(Register, Label),
    Jalr(Register, Register, i32),
    Branch(BranchCondition, Register, Register, Label),
    Load(LoadWidth, Register, Address),
    Store(StoreWidth, Register, Address),
    IntegerImmediate(IntegerImmediateOperation, Register, Register, i32),
    IntegerRegister(IntegerRegisterOperation, Register, Register, Register),
}

impl Instruction {
    pub fn from_pest_pair(pair: pest::iterators::Pair<riscv_parser::Rule>) -> Self {
        assert_eq!(pair.as_rule(), riscv_parser::Rule::instruction);
        let mut pairs = pair.into_inner();
        let instruction_name = pairs.next().unwrap();
        let argument_list: Vec<_> = pairs.collect();
        match instruction_name.as_str() {
            "lui" => {
                let rd = Register::from_pest_pair(&argument_list[0]);
                let imm = argument_list[1].as_str().parse::<u32>().unwrap();
                Self::Lui(rd, imm)
            }
            "auipc" => {
                let rd = Register::from_pest_pair(&argument_list[0]);
                let imm = argument_list[1].as_str().parse::<u32>().unwrap();
                Self::Auipc(rd, imm)
            }
            _ => panic!("Unsupported instruction")
        }
    }
}

pub struct Label {
    name: String,
}

impl Label {
    pub fn from_pest_pair(pair: pest::iterators::Pair<riscv_parser::Rule>) -> Self {
        assert_eq!(pair.as_rule(), riscv_parser::Rule::label);
        let label_name = pair.into_inner().next().unwrap();
        Self { name: label_name.as_str().to_string() }
    }
}
