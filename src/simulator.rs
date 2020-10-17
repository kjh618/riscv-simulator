use crate::ast;
use std::collections::HashMap;
use std::fmt;

// TODO: Seperate structs for register and memory
pub struct State<'a> {
    pc: u32,
    register: [u32; 32],
    instruction_memory: [ast::Instruction<'a>; 1024],
    data_memory: [u8; 16 * 1024],
    label_to_address: HashMap<&'a str, u32>,
}

impl fmt::Display for State<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "pc: {}", self.pc)?;
        writeln!(f, "registers: {:?}", self.register)?;
        // TODO
        write!(f, "others: ...")
    }
}

fn address_add_offset(address: u32, offset: i32) -> u32 {
    ((address as i64) + (offset as i64)) as u32
}

impl<'a> State<'a> {
    pub fn new(program: ast::Program<'a>) -> Self {
        let mut register = [0u32; 32];
        register[2] = 0xF00;
        let mut address = 0u32;
        let mut instruction_memory = [ast::Instruction::Nop; 1024];
        let mut label_to_address = HashMap::new();
        for line in program.lines {
            match line {
                ast::Line::Instruction(instruction) => {
                    instruction_memory[(address / 4) as usize] = instruction;
                    address += 4;
                }
                ast::Line::Label(label) => {
                    label_to_address.insert(label.name, address);
                }
            };
        }

        Self {
            pc: 0,
            register,
            instruction_memory,
            data_memory: [0; 16 * 1024],
            label_to_address,
        }
    }

    pub fn next(&mut self) {
        let instruction = self.instruction_memory[(self.pc / 4) as usize];
        match instruction {
            ast::Instruction::Lui(dest, immediate) => {
                self.register[dest.index as usize] = immediate << 12;
            }
            ast::Instruction::Auipc(dest, immediate) => {
                self.register[dest.index as usize] = self.pc + (immediate << 12);
            }
            ast::Instruction::Jal(dest, target) => {
                let target_address = match target {
                    ast::JumpTarget::Label(label_name) => self.label_to_address[label_name],
                    ast::JumpTarget::Offset(offset) => address_add_offset(self.pc, offset),
                };
                self.register[dest.index as usize] = self.pc + 4;
                self.pc = target_address;
            }
            ast::Instruction::Jalr(dest, base, offset) => {
                let target_address = address_add_offset(self.register[base.index as usize], offset) & !0x1;
                self.register[dest.index as usize] = self.pc + 4;
                self.pc = target_address;
            }
            _ => panic!(),
        }
    }
}
