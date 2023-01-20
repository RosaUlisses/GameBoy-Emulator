const REGISTER_COUNT: usize = 8;
const MEMORY_SIZE: usize = 65536; 

pub enum Flags {
    Z = 7,
    N = 6,
    H = 5, 
    C = 4
}

#[derive(Clone, Copy)]
pub enum Registers8bit {
    A = 0, 
    F = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    H = 6,
    L = 7
}

#[derive(Clone, Copy)]
pub enum Registers16bit {
    AF = 0, 
    BC = 1,
    DE = 2,
    HL = 3,
    SP = 4,
    PC = 5
}

pub struct CPU {
    pub registers: [u8; REGISTER_COUNT],
    pub stack_pointer: u16,
    pub program_counter: u16,
    pub memory: [u8; MEMORY_SIZE]
}

impl CPU {
    pub fn new() -> Self {
        return CPU {
            registers: [0; 8],
            stack_pointer: 0,
            program_counter: 0,
            memory: [0; MEMORY_SIZE]
        };
    }

    pub fn get_memory_8bit(&self, address: u16) -> u8{
        return self.memory[address as usize]; 
    }

    pub fn set_memory_8bit(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn get_memory_16bit(&self, address: u16) -> u16{
        return self.get_memory_8bit(address) as u16
            | (self.get_memory_8bit(address + 1) as u16) << 8; 
    }

    pub fn set_memory_16bit(&mut self, address: u16, value: u16) {
        self.set_memory_8bit(address, value as u8);
        self.set_memory_8bit(address + 1, (value >> 8) as u8);
    }
    
    pub fn get_8bit_memory_from_sp(&mut self) -> u8 {
        return self.get_memory_8bit(self.stack_pointer);
    }
    
    pub fn set_8bit_memory_from_sp(&mut self, value: u8) {
        self.set_memory_8bit(self.stack_pointer, value);
    }
    
    pub fn get_16bit_memory_from_sp(&mut self) -> u16 {
        return self.get_memory_16bit(self.stack_pointer);
    }
    
    pub fn set_16bit_memory_from_sp(&mut self, value: u16) {
        self.set_memory_16bit(self.stack_pointer, value);
    }
    
    pub fn get_8bit_memory_from_pc(&mut self) -> u8 {
        return self.get_memory_8bit(self.program_counter);
    }
    
    pub fn set_8bit_memory_from_pc(&mut self, value: u8) {
        self.set_memory_8bit(self.program_counter, value);
    }

    pub fn get_16bit_memory_from_pc(&mut self) -> u16 {
        return self.get_memory_16bit(self.program_counter);
    }
    
    pub fn set_16bit_memory_from_pc(&mut self, value: u16) {
        self.set_memory_16bit(self.program_counter, value);
    }
    
    pub fn get_8bit_memory_from_register(&mut self, register: Registers16bit) -> u8 {
        let address = self.get_register_16bit(register);
        return self.get_memory_8bit(address);
    }
    
    pub fn set_8bit_memory_from_register(&mut self, value: u8, register: Registers16bit) {
        let address = self.get_register_16bit(register);
        self.set_memory_8bit(address, value);
    }
    
    pub fn get_16bit_memory_from_register(&mut self, register: Registers16bit) -> u16 {
        let address = self.get_register_16bit(register);
        return self.get_memory_16bit(address);
    }
    
    pub fn set_16bit_memory_from_register(&mut self, value: u16, register: Registers16bit) {
        let address = self.get_register_16bit(register);
        self.set_memory_16bit(address, value);
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        return self.get_register_8bit(Registers8bit::F)
            >> (flag as u8) & 1 == 1;
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        if value {
            self.registers[Registers8bit::F as usize] |= 1 << flag as u8;
        }
        else {
            self.registers[Registers8bit::F as usize] &= !(1 << flag as u8);
        }
    }

    pub fn get_register_8bit(&self, register: Registers8bit) -> u8 {
        return self.registers[register as usize]; 
    }

    pub fn set_register_8bit(&mut self, register: Registers8bit, value: u8) {
        self.registers[register as usize] = value;
    }

    pub fn get_register_16bit(&self, register: Registers16bit) -> u16 {
        let index: usize = register as usize * 2;
        return (self.registers[index] as u16) << 8
              | self.registers[index + 1] as u16;
    }

    pub fn set_register_16bit(&mut self, register: Registers16bit, value: u16) {
        let index: usize = register as usize * 2;
        self.registers[index] = (value >> 8) as u8; 
        self.registers[index + 1] = value as u8; 
    }

    pub fn get_sp(&self) -> u16 {
        return self.stack_pointer;
    }

    pub fn set_sp(&mut self, value: u16) {
        self.stack_pointer = value;
    }

    pub fn get_pc(&self) -> u16 {
        return self.program_counter;
    }
    pub fn set_pc(&mut self, value: u16) {
        self.program_counter = value;
    }

}

