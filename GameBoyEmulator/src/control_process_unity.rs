const REGISTER_COUNT : usize = 8;
const MEMORY_SIZE : usize = 65536; 

pub enum flags {
    Z = 7,
    N = 6,
    H = 5, 
    C = 4
}

pub enum registers {
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

    pub fn set_flag(&mut self, flag : flags) {
       self.registers[registers::F as usize] = self.registers[registers::F as usize] | (1 << flag as u8); 
    }

    pub fn reset_flag(&mut self, flag : flags) {
       self.registers[registers::F as usize] = self.registers[registers::F as usize] | !(1 << flag as u8); 
    }
}

