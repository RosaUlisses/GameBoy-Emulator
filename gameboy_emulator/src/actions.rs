use crate::cpu::CPU;
use crate::cpu::Flags;
use crate::cpu::Registers16bit;
use crate::cpu::Registers8bit;

pub enum Operand8bit {
    Register(Registers8bit),
    Immediate(u8),
    Address(u16)
}

impl Operand8bit {
    pub fn get(&self, cpu : &CPU) -> u8 {
        match self {
            Operand8bit::Register(register) => return cpu.get_register(*register),
            Operand8bit::Immediate(immediate) => return *immediate,
            Operand8bit::Address(address) => return cpu.get_memory_adress_8bit(*address),
        }
    }
    pub fn set(&self, cpu : &mut CPU, value : u8) {
        match self {
            Operand8bit::Register(register) => cpu.set_register(*register, value),
            Operand8bit::Immediate(_) => (),
            Operand8bit::Address(address) => cpu.set_memory_adress_8bit(*address, value),
        }
    }
}

pub enum Operand16bit {
    Register(Registers16bit),
    Immediate(u16),
    Address(u16),
}

impl Operand16bit {
    pub fn get(&self, cpu : &mut CPU) -> u16 {
        match self {
            Operand16bit::Register(register) => cpu.get_16bit_register(*register),
            Operand16bit::Immediate(immediate) => *immediate,
            Operand16bit::Address(address) => cpu.get_memory_adress_16bit(*address),
        }
    }
    pub fn set(&self, cpu : &mut CPU, value : u16) {
        match self {
            Operand16bit::Register(register) => cpu.set_16bit_register(*register, value),
            Operand16bit::Immediate(_) => (),
            Operand16bit::Address(address) => cpu.set_memory_adress_16bit(*address, value),
        }
    }
}

pub fn ld(cpu : &mut CPU, operand1 : Operand8bit, operand2 : Operand8bit) {
    let value = operand2.get(cpu);
    operand1.set(cpu, operand2.get(cpu));
}

pub fn ld_16bit(cpu : &mut CPU, operand1 : Operand16bit, operand2 : Operand16bit){
    let value = operand2.get(cpu);
    operand1.set(cpu, value);
}

pub fn push(cpu : &mut CPU, operand1 : Operand16bit) {
    let value = operand1.get(cpu);
    cpu.set_memory_addressed_by_sp((value << 8) as u8);
    cpu.stack_pointer -= 1;
    cpu.set_memory_addressed_by_sp(value as u8);
    cpu.stack_pointer -= 1;
}

pub fn pop(cpu : &mut CPU, operand1 : Operand16bit) {
    let value1 = cpu.get_memory_addressed_by_sp() as u16;
    cpu.stack_pointer += 1;
    let value2 = cpu.get_memory_addressed_by_sp() as u16;
    cpu.stack_pointer += 1;

    operand1.set(cpu, value1 | (value2 >> 8))
}

pub fn add(cpu : &mut CPU, operand1 : Operand8bit) {
    let value1 : u16 = cpu.get_register(Registers8bit::A) as u16;
    let value2 : u16 = operand1.get(cpu) as u16;

    let sum : u16 = value1 + value2;
    cpu.set_register(Registers8bit::A, sum as u8);
    
    cpu.set_flag(Flags::Z, sum == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, (value1 & 0x0F) + (value2 & 0x0F) >= 0x10);
    cpu.set_flag(Flags::C, sum >= 0x100);
}

pub fn adc(cpu : &mut CPU, operand1 : Operand8bit) {
    let value1 : u16 = cpu.get_register(Registers8bit::A) as u16;
    let value2 : u16 = operand1.get(cpu) as u16;

    let sum : u16;
    let carry : u16;
    if cpu.get_flag(Flags::C){
        carry = 1;
    }
    else {
        carry = 0;
    } 
    sum = value1 + value2 + carry;
    cpu.set_register(Registers8bit::A, sum as u8);
    
    cpu.set_flag(Flags::Z, sum == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, (value1 & 0x0F) + (value2 & 0x0F) + carry >= 0x10);
    cpu.set_flag(Flags::C, sum >= 0x100);
}

pub fn sub(cpu : &mut CPU, operand1 : Operand8bit) {
    let value1 : u16 = cpu.get_register(Registers8bit::A) as u16;
    let value2 : u16 = operand1.get(cpu) as u16;
    let difference : u16 = value1 - value2;

    cpu.set_register(Registers8bit::A, difference as u8);

    cpu.set_flag(Flags::Z, difference == 0);
    cpu.set_flag(Flags::N, true);
    cpu.set_flag(Flags::H, (value1 & 0x0F) < (value2 & 0x0F));
    cpu.set_flag(Flags::C, value1 < value2);
}

pub fn sbc(cpu : &mut CPU, operand1 : Operand8bit) {
    let value1 : u16 = cpu.get_register(Registers8bit::A) as u16;
    let value2 : u16 = operand1.get(cpu) as u16;

    let difference : u16;
    let carry : u16;
    if cpu.get_flag(Flags::C){
        carry = 1;
    }
    else {
        carry = 0;
    }

    difference = value1 - value2 + carry;
    cpu.set_register(Registers8bit::A, difference as u8);

    cpu.set_flag(Flags::Z, difference == 0);
    cpu.set_flag(Flags::N, true);
    cpu.set_flag(Flags::H, (value1 & 0x0F) < (value2 & 0x0F));
    cpu.set_flag(Flags::C, value1 < value2);
}



pub fn and(cpu : &mut CPU, operand1 : Operand8bit){
    let result = cpu.get_register(Registers8bit::A) & operand1.get(cpu);

    cpu.set_register(Registers8bit::A, result);

    cpu.set_flag(Flags::Z, result == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, true);
    cpu.set_flag(Flags::C, false);

}

pub fn or(cpu : &mut CPU, operand1 : Operand8bit){
    let result = cpu.get_register(Registers8bit::A) | operand1.get(cpu);

    cpu.set_register(Registers8bit::A, result);

    cpu.set_flag(Flags::Z, result == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, false);

}

pub fn xor(cpu : &mut CPU, operand1 : Operand8bit){
    let result = cpu.get_register(Registers8bit::A) ^ operand1.get(cpu);

    cpu.set_register(Registers8bit::A, result);

    cpu.set_flag(Flags::Z, result == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, false);
    cpu.set_flag(Flags::C, false);

}

pub fn cp(cpu : &mut CPU, operand1 : Operand8bit) {
    let value1 : u8 = cpu.get_register(Registers8bit::A) as u8;
    let value2 : u8 = operand1.get(cpu) as u8;

    cpu.set_flag(Flags::Z, value1 == value2);
    cpu.set_flag(Flags::N, true);
    cpu.set_flag(Flags::H, (value1 & 0x0F) < (value2 & 0x0F));
    cpu.set_flag(Flags::C, value1 < value2);
}

pub fn inc(cpu : &mut CPU, operand1 : Operand8bit) {
   let incremented : u8 = operand1.get(cpu) + 1;

   operand1.set(cpu, incremented);

   cpu.set_flag(Flags::Z, incremented == 0);
   cpu.set_flag(Flags::N, false);
   cpu.set_flag(Flags::H, (incremented & 0x0F) == 0);
}

pub fn dec(cpu : &mut CPU, operand1 : Operand8bit) {
   let decremented : u8 = operand1.get(cpu) - 1;

   operand1.set(cpu,decremented);

   cpu.set_flag(Flags::Z, decremented == 0);
   cpu.set_flag(Flags::N, true);
   cpu.set_flag(Flags::H, (decremented & 0x0F) != 0xF);
}

pub fn addhl(cpu : &mut CPU, operand1 : Operand16bit) {
    let value1 = cpu.get_HL() as u32;
    let value2 = operand1.get(cpu) as u32;

    let sum = value1 + value2;
    cpu.set_HL(sum as u16);

    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, (value1 & 0x0FFF) + (value2 & 0x0FFF) >= 0x1000);
    cpu.set_flag(Flags::C, sum >= 0x10000);
}

pub fn addsp(cpu : &mut CPU, operand1 : Operand16bit) {
    let value1 = cpu.stack_pointer as u32;
    let value2 = operand1.get(cpu) as u32;

    let sum = value1 + value2;
    cpu.set_SP(sum as u16);

    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, (value1 & 0x0FFF) + (value2 & 0x0FFF) >= 0x1000);
    cpu.set_flag(Flags::C, sum >= 0x10000);
}

pub fn inc16bit(cpu : &mut CPU, operand1 : Operand16bit){
    let value = operand1.get(cpu);
    operand1.set(cpu, value)
}

pub fn dec16bit(cpu : &mut CPU, operand1 : Operand16bit){
    let value = operand1.get(cpu);
}


