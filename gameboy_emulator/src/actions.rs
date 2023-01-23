use crate::cpu::CPU;
use crate::cpu::Flags;
use crate::cpu::Registers16bit;
use crate::cpu::Registers8bit;

use crate::bitwise;

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

pub fn ld(cpu: &mut CPU, operand1: Operand8bit, operand2: Operand8bit) {
    let value = operand2.get(cpu);
    operand1.set(cpu, value);
}

pub fn ldd_ahl(cpu: &mut CPU) {
    let value = cpu.get_8bit_memory_from_register(Registers16bit::HL);  
    cpu.set_register_8bit(Registers8bit::A, value);
    dec16bit(cpu, Operand16bit::Register(Registers16bit::HL));
}

pub fn ldd_hla(cpu : &mut CPU) {
    let value = cpu.get_register_8bit(Registers8bit::A);
    cpu.set_8bit_memory_from_register(value, Registers16bit::HL);
    dec16bit(cpu, Operand16bit::Register(Registers16bit::HL));
}

pub fn ld_16bit(cpu: &mut CPU, operand1: Operand16bit, operand2: Operand16bit) {
    let value = operand2.get(cpu);
    operand1.set(cpu, value);
}

/*
    (The Stack Pointer automatically decrements before it puts something
    onto the stack so it is perfectly acceptable to assign it a value
    which points to a memory address which is one location past the end
    of available RAM.)
*/
pub fn push(cpu: &mut CPU, operand1: Operand16bit) {
    let value = operand1.get(cpu);
    cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(2);

    cpu.set_16bit_memory_from_sp(value);
}

pub fn pop(cpu: &mut CPU, operand1: Operand16bit) {
    let value = cpu.get_16bit_memory_from_sp();
    cpu.stack_pointer = cpu.stack_pointer.wrapping_add(2);

    operand1.set(cpu, value);
}

pub fn add(cpu: &mut CPU, operand1: Operand8bit) {
    let value1: u16 = cpu.get_register_8bit(Registers8bit::A) as u16;
    let value2: u16 = operand1.get(cpu) as u16;

    let sum: u16 = value1.wrapping_add(value2);
    cpu.set_register_8bit(Registers8bit::A, sum as u8);
    
    cpu.set_flag(Flags::Z, sum == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, (value1 & 0x0F) + (value2 & 0x0F) >= 0x10);
    cpu.set_flag(Flags::C, sum >= 0x100);
}

pub fn adc(cpu: &mut CPU, operand1: Operand8bit) {
    let value1: u16 = cpu.get_register_8bit(Registers8bit::A) as u16;
    let value2: u16 = operand1.get(cpu) as u16;

    let carry: u16 = cpu.get_flag(Flags::C) as u16; // 1 or 0
    let sum = value1.wrapping_add(value2).wrapping_add(carry);
    cpu.set_register_8bit(Registers8bit::A, sum as u8);
    
    cpu.set_flag(Flags::Z, sum == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, (value1 & 0x0F) + (value2 & 0x0F) + carry >= 0x10);
    cpu.set_flag(Flags::C, sum >= 0x100);
}

pub fn sub(cpu: &mut CPU, operand1: Operand8bit) {
    let value1: u16 = cpu.get_register_8bit(Registers8bit::A) as u16;
    let value2: u16 = operand1.get(cpu) as u16;

    let difference: u16 = value1.wrapping_sub(value2);
    cpu.set_register_8bit(Registers8bit::A, difference as u8);

    cpu.set_flag(Flags::Z, difference == 0);
    cpu.set_flag(Flags::N, true);
    cpu.set_flag(Flags::H, (value1 & 0x0F) < (value2 & 0x0F));
    cpu.set_flag(Flags::C, value1 < value2);
}

pub fn sbc(cpu: &mut CPU, operand1: Operand8bit) {
    let value1: u16 = cpu.get_register_8bit(Registers8bit::A) as u16;
    let value2: u16 = operand1.get(cpu) as u16;

    let carry: u16 = cpu.get_flag(Flags::C) as u16; // 1 or 0
    let difference = value1.wrapping_sub(value2).wrapping_add(carry);
    cpu.set_register_8bit(Registers8bit::A, difference as u8);

    cpu.set_flag(Flags::Z, difference == 0);
    cpu.set_flag(Flags::N, true);
    cpu.set_flag(Flags::H, (value1 & 0x0F) < (value2 & 0x0F));
    cpu.set_flag(Flags::C, value1 < value2);
}

pub fn and(cpu: &mut CPU, operand1: Operand8bit) {
    let result = cpu.get_register_8bit(Registers8bit::A) & operand1.get(cpu);

    cpu.set_register_8bit(Registers8bit::A, result);

    cpu.set_flag(Flags::Z, result == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, true);
    cpu.set_flag(Flags::C, false);

}

pub fn or(cpu: &mut CPU, operand1: Operand8bit) {
    let result = cpu.get_register_8bit(Registers8bit::A) | operand1.get(cpu);

    cpu.set_register_8bit(Registers8bit::A, result);

    cpu.set_flag(Flags::Z, result == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, false);

}

pub fn xor(cpu: &mut CPU, operand1: Operand8bit) {
    let result = cpu.get_register_8bit(Registers8bit::A) ^ operand1.get(cpu);

    cpu.set_register_8bit(Registers8bit::A, result);

    cpu.set_flag(Flags::Z, result == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, false);

}

pub fn cp(cpu: &mut CPU, operand1: Operand8bit) {
    let value1: u8 = cpu.get_register_8bit(Registers8bit::A) as u8;
    let value2: u8 = operand1.get(cpu) as u8;

    cpu.set_flag(Flags::Z, value1 == value2);
    cpu.set_flag(Flags::N, true);
    cpu.set_flag(Flags::H, (value1 & 0x0F) < (value2 & 0x0F));
    cpu.set_flag(Flags::C, value1 < value2);
}

pub fn inc(cpu: &mut CPU, operand1: Operand8bit) {
   let incremented: u8 = operand1.get(cpu).wrapping_add(1);

   operand1.set(cpu, incremented);

   cpu.set_flag(Flags::Z, incremented == 0);
   cpu.set_flag(Flags::N, false);
   cpu.set_flag(Flags::H, (incremented & 0x0F) == 0);
}

pub fn dec(cpu: &mut CPU, operand1: Operand8bit) {
   let decremented: u8 = operand1.get(cpu).wrapping_sub(1);

   operand1.set(cpu,decremented);

   cpu.set_flag(Flags::Z, decremented == 0);
   cpu.set_flag(Flags::N, true);
   cpu.set_flag(Flags::H, (decremented & 0x0F) != 0xF);
}

pub fn addhl(cpu: &mut CPU, operand1: Operand16bit) {
    let value1 = cpu.get_register_16bit(Registers16bit::HL) as u32;
    let value2 = operand1.get(cpu) as u32;

    let sum = value1 + value2;
    cpu.set_register_16bit(Registers16bit::HL, sum as u16);

    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, (value1 & 0x0FFF) + (value2 & 0x0FFF) >= 0x1000);
    cpu.set_flag(Flags::C, sum >= 0x10000);
}

pub fn addsp(cpu: &mut CPU, operand1: Operand16bit) {
    let value1 = cpu.get_sp() as u32;
    let value2 = operand1.get(cpu) as u32;

    let sum = value1 + value2;
    cpu.set_sp(sum as u16);

    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, (value1 & 0x0FFF) + (value2 & 0x0FFF) >= 0x1000);
    cpu.set_flag(Flags::C, sum >= 0x10000);
}

pub fn inc16bit(cpu: &mut CPU, operand1: Operand16bit) {
    let value = operand1.get(cpu).wrapping_add(1);
    operand1.set(cpu, value)
}

pub fn dec16bit(cpu: &mut CPU, operand1: Operand16bit) {
    let value = operand1.get(cpu).wrapping_sub(1);
    operand1.set(cpu, value)
}

pub fn swap(cpu: &mut CPU, operand1: Operand8bit) {
    let value = (operand1.get(cpu) << 4) | (operand1.get(cpu) >> 4);
    operand1.set(cpu, value);

    cpu.set_flag(Flags::Z, value == 0);

    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, false);
}

pub fn daa(cpu: &mut CPU) {

    let mut a_value = cpu.get_register_8bit(Registers8bit::A);

    if !cpu.get_flag(Flags::N) {
       
        if cpu.get_flag(Flags::C) || (a_value > 0x99) {
            a_value = a_value.wrapping_add(0x60);
            cpu.set_flag(Flags::C, true);
        }
        if cpu.get_flag(Flags::H) || ((a_value & 0x0f) > 0x09) {
            a_value = a_value.wrapping_add(0x06);
        } 
    }
    else {
        if cpu.get_flag(Flags::C) {
            a_value = a_value.wrapping_sub(0x60);
        }
        if cpu.get_flag(Flags::H) {
            a_value = a_value.wrapping_sub(0x06);
        }
    }

    cpu.set_flag(Flags::Z, a_value == 0);
    cpu.set_flag(Flags::H, false);
}

pub fn cpl(cpu: &mut CPU) {
    let value = cpu.get_register_8bit(Registers8bit::A);
    cpu.set_register_8bit(Registers8bit::A, !value);

    cpu.set_flag(Flags::N, true);
    cpu.set_flag(Flags::H, true);
}

pub fn ccf(cpu: &mut CPU) {
    cpu.set_flag(Flags::C, !cpu.get_flag(Flags::C));

    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
}

pub fn scf(cpu: &mut CPU) {
    cpu.set_flag(Flags::C, true);

    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
}

pub fn nop() {
    return;
}

pub fn rlc(cpu: &mut CPU, operand1: Operand8bit) {
    let a_value = operand1.get(cpu); 
    let shifted_value = a_value << 1;
    
    operand1.set(cpu, shifted_value);

    cpu.set_flag(Flags::Z, shifted_value == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, (a_value & 1) == 1);
}

pub fn rl(cpu: &mut CPU, operand1: Operand8bit) {
    let a_value = operand1.get(cpu);

    // isso da certo ?
    let c_flag = cpu.get_flag(Flags::C) as u8; 
    let shifted_value = (a_value << 1) | (c_flag >> 7);

    operand1.set(cpu, shifted_value);

    cpu.set_flag(Flags::Z, shifted_value == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, (a_value & 1) == 1);
}

pub fn rrc(cpu: &mut CPU, operand1: Operand8bit) {
    let a_value = operand1.get(cpu); 
    let shifted_value = a_value >> 1;
    operand1.set(cpu, shifted_value);

    cpu.set_flag(Flags::Z, shifted_value == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, (a_value & 0x80) == 1);
}

pub fn rr(cpu: &mut CPU, operand1: Operand8bit) {
    let a_value = operand1.get(cpu);
    // isso da certo ?
    let c_flag = cpu.get_flag(Flags::C) as u8; 
    let shifted_value = (a_value >> 1) | c_flag;

    operand1.set(cpu, shifted_value);

    cpu.set_flag(Flags::Z, shifted_value == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, (a_value & 0x80) == 1);
}

pub fn sla(cpu: &mut CPU, operand1: Operand8bit) {
    let a_value = operand1.get(cpu); 
    let shifted_value = (a_value << 1) & !(1);
    
    operand1.set(cpu, shifted_value);

    cpu.set_flag(Flags::Z, shifted_value == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, (a_value & 1) == 1);   
}

pub fn sra(cpu: &mut CPU, operand1: Operand8bit) {
    let a_value = operand1.get(cpu); 
    let shifted_value = (a_value >> 1) & !(0x80);
    operand1.set(cpu, shifted_value);

    cpu.set_flag(Flags::Z, shifted_value == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, (a_value & 0x80) == 1);
}

pub fn srl(cpu: &mut CPU, operand1: Operand8bit) {
    let a_value = operand1.get(cpu); 
    let shifted_value = a_value >> 1;
    operand1.set(cpu, shifted_value);

    cpu.set_flag(Flags::Z, shifted_value == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, (a_value & 0x80) == 1);
}

pub fn bit(cpu: &mut CPU, operand1: Operand8bit, bit: u8){
    let bit = bitwise::get_bit_8b(operand1.get(cpu), bit);

    cpu.set_flag(Flags::Z, bit);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, true);
}

pub fn set(cpu: &mut CPU, operand1: Operand8bit, bit: u8) {
    let value = operand1.get(cpu);
    operand1.set(cpu, bitwise::set_bit_8b(value, bit, true));
}

pub fn res(cpu: &mut CPU, operand1: Operand8bit, bit: u8) {
    let value = operand1.get(cpu);
    operand1.set(cpu, bitwise::set_bit_8b(value, bit, false));
}