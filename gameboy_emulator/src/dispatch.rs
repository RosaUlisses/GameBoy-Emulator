use crate::cpu::CPU;
use crate::cpu::Registers16bit;
use crate::cpu::Registers8bit;
use crate::instructions;

use crate::table::INSTRUCTIONS;

#[derive(Clone, Copy)]
pub enum Operand8bit {
    Register(Registers8bit),
    Immediate(u8),
    Address(u16)
}

impl Operand8bit {
    pub fn get(self, cpu: &CPU) -> u8 {
        match self {
            Operand8bit::Register(register)
                => return cpu.get_register_8bit(register),
            Operand8bit::Immediate(immediate)
                => return immediate,
            Operand8bit::Address(address)
                => return cpu.get_memory_8bit(address),
        }
    }

    pub fn set(self, cpu: &mut CPU, value: u8) {
        match self {
            Operand8bit::Register(register)
                => cpu.set_register_8bit(register, value),
            Operand8bit::Immediate(_)
                => panic!("operand.set() on an immediate value"),
            Operand8bit::Address(address)
                => cpu.set_memory_8bit(address, value),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Operand16bit {
    Register(Registers16bit),
    Immediate(u16),
    Address(u16),
}

impl Operand16bit {
    pub fn get(self, cpu: &mut CPU) -> u16 {
        match self {
            Operand16bit::Register(register)
                => cpu.get_register_16bit(register),
            Operand16bit::Immediate(immediate)
                => immediate,
            Operand16bit::Address(address)
                => cpu.get_memory_16bit(address),
        }
    }
    pub fn set(self, cpu: &mut CPU, value: u16) {
        match self {
            Operand16bit::Register(register)
                => cpu.set_register_16bit(register, value),
            Operand16bit::Immediate(_)
                => panic!("operand.set() on an immediate value"),
            Operand16bit::Address(address)
                => cpu.set_memory_16bit(address, value),
        }
    }
}

#[derive(Clone, Copy)]
pub enum AddressingMode8bit {
    Register(Registers8bit),
    Immediate,
    Address8bitImmediate,
    Address16bitImmediate,
    CAddress,
    HLAdddress,
    Fixed(u8),
}

#[derive(Clone, Copy)]
pub enum AddressingMode16bit {
    Register(Registers16bit),
    Immediate,
    Address,
    Fixed(u16),
}

impl AddressingMode8bit {
    pub fn fetch_operand(self, cpu: &mut CPU) -> Operand8bit {
        match self {
            AddressingMode8bit::Register(register)
                => Operand8bit::Register(register),
            AddressingMode8bit::Immediate
                => Operand8bit::Immediate(cpu.fetch_next_8bits_pc()),
            AddressingMode8bit::Address8bitImmediate
                => Operand8bit::Address(cpu.fetch_next_8bits_pc() as u16),
            AddressingMode8bit::Address16bitImmediate
                => Operand8bit::Address(cpu.fetch_next_16bits_pc()),
            AddressingMode8bit::CAddress
                => Operand8bit::Address(cpu.get_register_8bit(Registers8bit::C) as u16),
            AddressingMode8bit::HLAdddress 
                => Operand8bit::Address(cpu.get_register_16bit(Registers16bit::HL)),    
            AddressingMode8bit::Fixed(value)
                => Operand8bit::Immediate(value),
        }
    }
}

impl AddressingMode16bit {
    pub fn fetch_operand(self, cpu: &mut CPU) -> Operand16bit {
        match self {
            AddressingMode16bit::Register(register)
                => Operand16bit::Register(register),
            AddressingMode16bit::Immediate
                => Operand16bit::Immediate(cpu.fetch_next_16bits_pc()),
            AddressingMode16bit::Address
                => Operand16bit::Address(cpu.fetch_next_16bits_pc()),
            AddressingMode16bit::Fixed(value)
                => Operand16bit::Immediate(value),
        }
    }
}

type FnImplied = fn(&mut CPU);
type FnOp8bit = fn(&mut CPU, Operand8bit);
type FnOp8bit8bit = fn(&mut CPU, Operand8bit, Operand8bit);
type FnOp16bit = fn(&mut CPU, Operand16bit);
type FnOp16bit16bit = fn(&mut CPU, Operand16bit, Operand16bit);

#[derive(Clone, Copy)]
pub enum Instruction {
    Implied(FnImplied),
    Op8bit(FnOp8bit, AddressingMode8bit),
    Op8bit8bit(FnOp8bit8bit, AddressingMode8bit, AddressingMode8bit),
    Op16bit(FnOp16bit, AddressingMode16bit),
    Op16bit16bit(FnOp16bit16bit, AddressingMode16bit, AddressingMode16bit),
    Prefix,
}

impl Instruction {
    pub fn execute(self, cpu: &mut CPU) {
        match self {
            Instruction::Implied(instruction) => {
                instruction(cpu);
            }
            Instruction::Op8bit(instruction, operand1) => {
                let op1 = operand1.fetch_operand(cpu);
                instruction(cpu, op1);
            }
            Instruction::Op8bit8bit(instruction, operand1, operand2) => {
                let op1 = operand1.fetch_operand(cpu);
                let op2 = operand2.fetch_operand(cpu);
                instruction(cpu, op1, op2);
            }
            Instruction::Op16bit(instruction, operand1) => {
                let op1 = operand1.fetch_operand(cpu);
                instruction(cpu, op1);
            }
            Instruction::Op16bit16bit(instruction, operand1, operand2) => {
                let op1 = operand1.fetch_operand(cpu);
                let op2 = operand2.fetch_operand(cpu);
                instruction(cpu, op1, op2);
            }
            Instruction::Prefix => {
                let opcode = cpu.fetch_next_8bits_pc();
                Instruction::exec_extended(cpu, opcode);
            }
        }
    }

    pub fn exec_extended(cpu: &mut CPU, opcode: u8) {
        let register_operands : [Operand8bit; 8] = [
            Operand8bit::Register(Registers8bit::B),
            Operand8bit::Register(Registers8bit::C),
            Operand8bit::Register(Registers8bit::D),
            Operand8bit::Register(Registers8bit::E),
            Operand8bit::Register(Registers8bit::H),
            Operand8bit::Register(Registers8bit::L),
            Operand8bit::Address(cpu.get_register_16bit(Registers16bit::HL)),
            Operand8bit::Register(Registers8bit::A),
        ];
        const OPERATIONS : [FnOp8bit; 8] = [
            instructions::rlc,
            instructions::rrc,
            instructions::rl,
            instructions::rr,
            instructions::sla,
            instructions::sra,
            instructions::swap,
            instructions::srl
        ];

        let operation = opcode >> 6;
        let op1 = (opcode >> 3) & 0b111;
        let op2: usize = (opcode & 0b111) as usize;

        match operation {
            0 => OPERATIONS[op1 as usize](cpu, register_operands[op2]),
            1 => instructions::bit(cpu, register_operands[op2], op1),
            2 => instructions::res(cpu, register_operands[op2], op1),
            3 => instructions::set(cpu, register_operands[op2], op1),
            _ => unreachable!()
        }
    }
}

