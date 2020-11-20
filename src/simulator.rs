use crate::ast;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops;

#[derive(Debug)]
struct RegisterFile {
    registers: [u32; RegisterFile::NUM_REGISTERS],
}

#[derive(Debug)]
struct InstructionMemory<'a> {
    memory: [ast::Instruction<'a>; InstructionMemory::SIZE / 4],
}

#[derive(Debug)]
struct DataMemory {
    memory: [u8; DataMemory::SIZE],
}

#[derive(Debug)]
pub struct State<'a> {
    pc: u32,
    register: RegisterFile,
    instruction_memory: InstructionMemory<'a>,
    data_memory: DataMemory,
    label_to_address: HashMap<&'a str, u32>,
}

impl RegisterFile {
    const NUM_REGISTERS: usize = 32;

    fn new() -> Self {
        let mut registers = [0u32; Self::NUM_REGISTERS];
        registers[2] = 0xF00;
        Self { registers }
    }
}

impl ops::Index<u8> for RegisterFile {
    type Output = u32;

    fn index(&self, index: u8) -> &Self::Output {
        if index == 0 {
            &0
        } else {
            &self.registers[index as usize]
        }
    }
}

impl ops::IndexMut<u8> for RegisterFile {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

impl<'a> InstructionMemory<'a> {
    const SIZE: usize = 4 * 1024;

    fn new(program: ast::Program<'a>, label_to_address: &mut HashMap<&'a str, u32>) -> Self {
        let mut address = 0;
        let mut memory = [ast::Instruction::Nop; Self::SIZE / 4];
        for line in program.lines {
            match line {
                ast::Line::Instruction(instruction) => {
                    memory[(address / 4) as usize] = instruction;
                    address += 4;
                }
                ast::Line::Label(label) => {
                    label_to_address.insert(label.name, address);
                }
            };
        }
        Self { memory }
    }

    fn translate(address: u32) -> u32 {
        address & 0xFFF
    }
}

impl<'a> ops::Index<u32> for InstructionMemory<'a> {
    type Output = ast::Instruction<'a>;

    fn index(&self, address: u32) -> &Self::Output {
        &self.memory[(Self::translate(address) / 4) as usize]
    }
}

impl DataMemory {
    const SIZE: usize = 16 * 1024;

    fn new() -> Self {
        Self {
            memory: [0; Self::SIZE],
        }
    }

    fn translate(address: u32) -> u32 {
        address & 0x3FFF
    }

    fn load(&self, byte_enable: u8, address: u32) -> u32 {
        let from = Self::translate(address) as usize;
        let to = (Self::translate(address) + 4) as usize;
        let mut bytes: [u8; 4] = self.memory[from..to].try_into().unwrap();

        let mut mask = 0b0001u8;
        for i in 0..4 {
            if byte_enable & mask == 0 {
                bytes[i] = 0;
            }
            mask <<= 1;
        }

        u32::from_le_bytes(bytes)
    }

    fn store(&mut self, byte_enable: u8, address: u32, value: u32) {
        let bytes = value.to_le_bytes();

        let mut mask = 0b0001u8;
        for i in 0..4 {
            if byte_enable & mask == 0 {
                self.memory[(Self::translate(address) as usize) + i] = bytes[i];
            }
            mask <<= 1;
        }
    }
}

impl<'a> State<'a> {
    pub fn new(program: ast::Program<'a>) -> Self {
        let mut label_to_address = HashMap::new();

        Self {
            pc: 0,
            register: RegisterFile::new(),
            instruction_memory: InstructionMemory::new(program, &mut label_to_address),
            data_memory: DataMemory::new(),
            label_to_address,
        }
    }

    fn address_add_offset(address: u32, offset: i32) -> u32 {
        ((address as i64) + (offset as i64)) as u32
    }

    pub fn next(&mut self) {
        let instruction = self.instruction_memory[self.pc];
        match instruction {
            ast::Instruction::Lui(dest, immediate) => {
                self.register[dest.index] = immediate << 12;
            }
            ast::Instruction::Auipc(dest, immediate) => {
                self.register[dest.index] = self.pc + (immediate << 12);
            }
            ast::Instruction::Jal(dest, target) => {
                let target_address = match target {
                    ast::JumpTarget::Label(label_name) => self.label_to_address[label_name],
                    ast::JumpTarget::Offset(offset) => Self::address_add_offset(self.pc, offset),
                };
                self.register[dest.index] = self.pc + 4;
                self.pc = target_address;
            }
            ast::Instruction::Jalr(dest, base, offset) => {
                let target_address = Self::address_add_offset(self.register[base.index], offset) & !0x1;
                self.register[dest.index] = self.pc + 4;
                self.pc = target_address;
            }
            // TODO
            _ => panic!(),
        };
    }
}

