use crate::cpu::CPU;
use crate::cpu::Registers16bit;
use crate::cpu::Registers8bit;


#[derive(Clone, Copy)]
pub enum Operand8bit {
    Register(Registers8bit),
    Immediate(u8),
    Address(u16)
}

enum AddressingMode8bit {
    Register(Registers8bit),
    Immediate,
    Address,
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
                => (),
            Operand8bit::Address(address)
                => cpu.set_memory_8bit(address, value),
        }
    }
}

impl AddressingMode8bit {
    pub fn fetch_operand(self, cpu: &mut CPU) -> Operand8bit {
        match self {
            AddressingMode8bit::Register(register)
                => Operand8bit::Register(register),
            AddressingMode8bit::Immediate
                => Operand8bit::Immediate(cpu.fetch_next_8bits_pc()),
            AddressingMode8bit::Address
                => Operand8bit::Address(cpu.fetch_next_16bits_pc()),
        }
    }
}

pub enum Operand16bit {
    Register(Registers16bit),
    Immediate(u16),
    Address(u16),
}

enum AddressingMode16bit {
    Register(Registers16bit),
    Immediate,
    Address,
}

impl Operand16bit {
    pub fn get(&self, cpu: &mut CPU) -> u16 {
        match self {
            Operand16bit::Register(register)
                => cpu.get_register_16bit(*register),
            Operand16bit::Immediate(immediate)
                => *immediate,
            Operand16bit::Address(address)
                => cpu.get_memory_16bit(*address),
        }
    }
    pub fn set(&self, cpu: &mut CPU, value: u16) {
        match self {
            Operand16bit::Register(register)
                => cpu.set_register_16bit(*register, value),
            Operand16bit::Immediate(_)
                => (),
            Operand16bit::Address(address)
                => cpu.set_memory_16bit(*address, value),
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
        }
    }
}

// enum
