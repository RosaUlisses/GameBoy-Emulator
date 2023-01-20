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
    pub memory : [u8; MEMORY_SIZE]
}

impl CPU {
    pub fn new() -> Self {
        return CPU {registers : [0; 8], stack_pointer : 0, program_counter : 0, memory : [0; MEMORY_SIZE]};
    }

    pub fn get_memory_adress(&self, address : u16) -> u8{
        return self.memory[address as usize]; 
    }

    pub fn set_memory_adress(&mut self, address : u16, value : u8) {
        self.memory[address as usize] = value;
    }

    pub fn get_flag(&self, flag : Flags) -> bool {
        self.get_register(Registers::F) >> (flag as u8) & 1 == 1
    }

    pub fn set_flag(&mut self, flag : Flags, value : bool) {
        if value {
            self.set_register(Registers::F, self.get_register(Registers::F) | (1 << flag as u8)); 
        }
        else {
            self.set_register(Registers::F, self.get_register(Registers::F) & !(1 << flag as u8)); 
        }
    }

    pub fn get_register(&self, register : Registers) -> u8 {
        return self.registers[register as usize]; 
    }

    pub fn set_register(&mut self, register : Registers, value : u8) {
        self.registers[register as usize] = value;
    }

    pub fn get_AF_value(&self) -> u16 {
        return ((self.get_register(Registers::A) << 7) | self.get_register(Registers::F)) as u16;
    }

    pub fn get_BC_value(&self) -> u16 {
        return ((self.get_register(Registers::B) << 7) | self.get_register(Registers::C)) as u16;
    }

    pub fn get_DE_value(&self) -> u16 {
        return ((self.get_register(Registers::B) << 7) | self.get_register(Registers::C)) as u16;
    }

    pub fn get_HL_value(&self) -> u16 {
        return ((self.get_register(Registers::H) << 7) | self.get_register(Registers::L)) as u16;
    }


}

