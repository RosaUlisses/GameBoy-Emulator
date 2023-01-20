const REGISTER_COUNT : usize = 8;
const MEMORY_SIZE : usize = 65536; 

pub enum Flags {
    Z = 7,
    N = 6,
    H = 5, 
    C = 4
}

pub enum Registers {
    A = 0, 
    F = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    H = 6,
    L = 7
}

pub struct CPU {
    pub registers : [u8; REGISTER_COUNT],
    pub stack_pointer : u16,
    pub program_counter : u16,
    pub memory : [u16; MEMORY_SIZE]
}

impl CPU {
    pub fn new() -> Self {
        return CPU {registers : [0; 8], stack_pointer : 0, program_counter : 0, memory : [0; MEMORY_SIZE]};
    }

    pub fn set_flag(&mut self, flag : Flags) {
       self.registers[Registers::F as usize] = self.registers[Registers::F as usize] | (1 << flag as u8); 
    }

    pub fn reset_flag(&mut self, flag : Flags) {
       self.registers[Registers::F as usize] = self.registers[Registers::F as usize] | !(1 << flag as u8); 
    }

    pub fn get_register(&self, register : Registers) -> u8 {
        match(register){
            Registers::A => return self.registers[Registers::A as usize],
            Registers::F => return self.registers[Registers::F as usize],
            Registers::B => return self.registers[Registers::B as usize],
            Registers::C => return self.registers[Registers::C as usize],
            Registers::D => return self.registers[Registers::D as usize],
            Registers::E => return self.registers[Registers::E as usize],
            Registers::H => return self.registers[Registers::H as usize],
            Registers::L => return self.registers[Registers::L as usize],
        } 
    }

    pub fn set_register(&mut self, register : Registers, value : u8) {
        match(register){
            Registers::A => self.registers[Registers::A as usize] = value,
            Registers::F => self.registers[Registers::F as usize] = value,
            Registers::B => self.registers[Registers::B as usize] = value,
            Registers::C => self.registers[Registers::C as usize] = value,
            Registers::D => self.registers[Registers::D as usize] = value,
            Registers::E => self.registers[Registers::E as usize] = value,
            Registers::H => self.registers[Registers::H as usize] = value,
            Registers::L => self.registers[Registers::L as usize] = value,
        } 
    }

    pub fn get_AF_value(&self) -> u16 {
        return ((self.registers[Registers::A as usize] << 7) | self.registers[Registers::F as usize]) as u16;
    }

    pub fn get_BC_value(&self) -> u16 {
        return ((self.registers[Registers::B as usize] << 7) | self.registers[Registers::C as usize]) as u16;
    }

    pub fn get_DE_value(&self) -> u16 {
        return ((self.registers[Registers::B as usize] << 7) | self.registers[Registers::C as usize]) as u16;
    }

    pub fn get_HL_value(&self) -> u16 {
        return ((self.registers[Registers::H as usize] << 7) | self.registers[Registers::L as usize]) as u16;
    }


}

