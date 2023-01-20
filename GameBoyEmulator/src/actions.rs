use crate::control_proccess_unity::CPU;
use crate::control_proccess_unity::Flags;
use crate::control_proccess_unity::Registers;

pub enum Operand {
    Register(Registers),
    Immediate(u8),
    Address(u16)
}

impl Operand {
    pub fn get(self, cpu : &CPU) -> u8 {
        match self {
            Operand::Register(register) => cpu.get_register(register),
            Operand::Immediate(immediate) => immediate,
            Operand::Address(address) => cpu.get_memory_adress(address),
        }
    }
    pub fn set(self, cpu : &mut CPU, value : u8) {
        match self {
            Operand::Register(register) => cpu.set_register(register, value),
            Operand::Immediate(_) => (),
            Operand::Address(address) => cpu.set_memory_adress(address, value),
        }
    }
}

pub fn add(cpu : &mut CPU, operand1 : Operand) {
    let value1 : u16 = cpu.get_register(Registers::A) as u16;
    let value2 : u16 = operand1.get(cpu) as u16;

    let sum : u16 = value1 + value2;
    cpu.set_register(Registers::A, sum as u8);
    
    cpu.set_flag(Flags::Z, sum == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, (value1 & 0x0F) + (value2 & 0x0F) >= 0x10);
    cpu.set_flag(Flags::C, sum >= 0x100);
}

pub fn adc(cpu : &mut CPU, operand1 : Operand) {
    let value1 : u16 = cpu.get_register(Registers::A) as u16;
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
    cpu.set_register(Registers::A, sum as u8);
    
    cpu.set_flag(Flags::Z, sum == 0);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::H, (value1 & 0x0F) + (value2 & 0x0F) + carry >= 0x10);
    cpu.set_flag(Flags::C, sum >= 0x100);
}

pub fn sub(cpu : &mut CPU, operand1 : Operand) {
    let value1 : u16 = cpu.get_register(Registers::A) as u16;
    let value2 : u16 = operand1.get(cpu) as u16;
    let difference : u16 = value1 - value2;

    cpu.set_register(Registers::A, difference as u8);

    cpu.set_flag(Flags::Z, difference == 0);
    cpu.set_flag(Flags::N, true);
    cpu.set_flag(Flags::H, (value1 & 0x0F) < (value2 & 0x0F));
    cpu.set_flag(Flags::C, value1 < value2);
}

pub fn sbc(cpu : &mut CPU, operand1 : Operand) {
    let value1 : u16 = cpu.get_register(Registers::A) as u16;
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
    cpu.set_register(Registers::A, difference as u8);

    cpu.set_flag(Flags::Z, difference == 0);
    cpu.set_flag(Flags::N, true);
    cpu.set_flag(Flags::H, (value1 & 0x0F) < (value2 & 0x0F));
    cpu.set_flag(Flags::C, value1 < value2);
}

