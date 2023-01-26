use serde_json::Map;
use std::fs;
use crate::cpu::CPU;
use crate::cpu::Registers16bit;
use crate::cpu::Registers8bit;
use crate::instructions;


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
                => (),
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
                => (),
            Operand16bit::Address(address)
                => cpu.set_memory_16bit(address, value),
        }
    }
}

#[derive(Clone, Copy)]
pub enum AddressingMode8bit {
    Register(Registers8bit),
    Immediate,
    Address(Registers16bit),
}

impl AddressingMode8bit {
    pub fn fetch_operand(self, cpu: &mut CPU) -> Operand8bit {
        match self {
            AddressingMode8bit::Register(register)
                => Operand8bit::Register(register),
            AddressingMode8bit::Immediate
                => Operand8bit::Immediate(cpu.fetch_next_8bits_pc()),
            AddressingMode8bit::Address(register)
                => Operand8bit::Address(cpu.get_register_16bit(register)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum AddressingMode16bit {
    Register(Registers16bit),
    Immediate,
    Address,
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
        }
    }
}

type JsonObject = serde_json::Value;
type Object = Option<Map<String, serde_json::value::Value>>;

pub fn instantiate_addressing_mode_8bit(literal: &str) -> AddressingMode8bit {
    match literal {
        "A" => AddressingMode8bit::Register(Registers8bit::A),
        "B" => AddressingMode8bit::Register(Registers8bit::B),
        "C" => AddressingMode8bit::Register(Registers8bit::C),
        "D" => AddressingMode8bit::Register(Registers8bit::D),
        "E" => AddressingMode8bit::Register(Registers8bit::E),
        "H" => AddressingMode8bit::Register(Registers8bit::H),
        "L" => AddressingMode8bit::Register(Registers8bit::L),
        "d8" => AddressingMode8bit::Immediate,
        "(BC)" => AddressingMode8bit::Address(Registers16bit::BC),
        "(HL)" => AddressingMode8bit::Address(Registers16bit::HL),
        "(DE)" => AddressingMode8bit::Address(Registers16bit::DE),
        &_ => todo!("Levantar excecao")
    }
}

pub fn instantiate_8bit_alu_instruction(instruction_json: &JsonObject) -> Instruction {
    let mnemonic  = instruction_json["mnemonic"].as_str().unwrap();
    let function: FnOp8bit;
    let addressing_mode = instantiate_addressing_mode_8bit(instruction_json["operand2"].as_str().unwrap());  
    match mnemonic {
        "ADD" => function = instructions::add,
        "SUB" => function = instructions::sub,
        "INC" => function = instructions::inc,
        "DEC" => function = instructions::dec,
        "ADC" => function = instructions::adc,
        "SBC" => function = instructions::sbc,
        "AND" => function = instructions::inc,
        &_ => todo!("Levantar excecao")
    }
    return Instruction::Op8bit(function, addressing_mode);
}

pub fn instantiate_instruction(instruction_json: &JsonObject) -> Instruction{

    let mnemonic  = instruction_json["mnemonic"].as_str().unwrap();

    match mnemonic {
        "ADD" => {
            let function = instructions::add;
            let addressing_mode = instantiate_addressing_mode_8bit(instruction_json["operand2"].as_str().unwrap());  
            Instruction::Op8bit(function, addressing_mode)
        } 
        "SUB" => {
            let function = instructions::sub;
            let addressing_mode = instantiate_addressing_mode_8bit(instruction_json["operand2"].as_str().unwrap());  
            Instruction::Op8bit(function, addressing_mode)
        }
        "INC" => {
            let function = instructions::inc;
            let addressing_mode = instantiate_addressing_mode_8bit(instruction_json["operand1"].as_str().unwrap());  
            Instruction::Op8bit(function, addressing_mode)
        }
        "DEC" => {
            let function = instructions::dec;
            let addressing_mode = instantiate_addressing_mode_8bit(instruction_json["operand1"].as_str().unwrap());  
            Instruction::Op8bit(function, addressing_mode)
        }
        "ADC" => {
            let function = instructions::adc;
            let addressing_mode = instantiate_addressing_mode_8bit(instruction_json["operand2"].as_str().unwrap());  
            Instruction::Op8bit(function, addressing_mode)
        }
        "SBC" => {
            let function = instructions::sbc;
            let addressing_mode = instantiate_addressing_mode_8bit(instruction_json["operand2"].as_str().unwrap());  
            Instruction::Op8bit(function, addressing_mode)
        }
        "AND" => {
            let function = instructions::inc;
            let addressing_mode = instantiate_addressing_mode_8bit(instruction_json["operand1"].as_str().unwrap());  
            Instruction::Op8bit(function, addressing_mode)
        }

        &_ => todo!("Levantar excecao")
    }
}

pub fn instaciate_unprefixed_instructions(instructions: JsonObject) {

    for (_, value) in instructions.as_object().unwrap() {
        instantiate_instruction(value);
    }
    
}

pub fn instantiate_instructions() {
   let text = fs::read_to_string("C:\\Programação\\GameBoy-Emulator\\gameboy_emulator\\src\\opcodes.json").
    expect("Error, it is not possible to read the file!");

   let opcodes : JsonObject = serde_json::from_str(&text).expect("JSON was not well-formed");
}