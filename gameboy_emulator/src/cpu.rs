const REGISTER_COUNT : usize = 8;
const MEMORY_SIZE : usize = 65536; 

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
    AF, 
    BC,
    DE,
    HL,
    SP,
    PC
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

    pub fn get_memory_adress_8bit(&self, address : u16) -> u8{
        return self.memory[address as usize]; 
    }

    pub fn set_memory_adress_8bit(&mut self, address : u16, value : u8) {
        self.memory[address as usize] = value;
    }

    pub fn get_memory_adress_16bit(&self, address : u16) -> u16{
        return (self.memory[address as usize] as u16) | ((self.memory[address as usize] << 8) as u16); 
    }

    pub fn set_memory_adress_16bit(&mut self, address : u16, value : u16) {
        self.memory[address as usize] = value as u8;
        self.memory[(address + 1) as usize] = (value << 8) as u8;
    }
    
    pub fn set_memory_addressed_by_sp(&mut self, value : u8) {
        self.memory[self.stack_pointer as usize] = value;
    }

    pub fn get_memory_addressed_by_sp(&mut self) -> u8 {
        return self.memory[self.stack_pointer as usize];
    }

    pub fn get_flag(&self, flag : Flags) -> bool {
        self.get_register(Registers8bit::F) >> (flag as u8) & 1 == 1
    }

    pub fn set_flag(&mut self, flag : Flags, value : bool) {
        if value {
            self.set_register(Registers8bit::F, self.get_register(Registers8bit::F) | (1 << flag as u8)); 
        }
        else {
            self.set_register(Registers8bit::F, self.get_register(Registers8bit::F) & !(1 << flag as u8)); 
        }
    }

    pub fn get_register(&self, register : Registers8bit) -> u8 {
        return self.registers[register as usize]; 
    }

    pub fn set_register(&mut self, register : Registers8bit, value : u8) {
        self.registers[register as usize] = value;
    }

    pub fn get_AF(&self) -> u16 {
        return ((self.get_register(Registers8bit::A) << 8) | self.get_register(Registers8bit::F)) as u16;
    }

    pub fn get_BC(&self) -> u16 {
        return ((self.get_register(Registers8bit::B) << 8) | self.get_register(Registers8bit::C)) as u16;
    }

    pub fn get_DE(&self) -> u16 {
        return ((self.get_register(Registers8bit::B) << 8) | self.get_register(Registers8bit::C)) as u16;
    }

    pub fn get_HL(&self) -> u16 {
        return ((self.get_register(Registers8bit::H) << 8) | self.get_register(Registers8bit::L)) as u16;
    }

    pub fn get_16bit_register(&self, register : Registers16bit) -> u16{
        match register {
            Registers16bit::AF =>  return self.get_AF(),
            Registers16bit::BC => return self.get_BC(),
            Registers16bit::DE => return self.get_DE(),
            Registers16bit::HL => return self.get_HL(),
            Registers16bit::SP => return self.stack_pointer,
            Registers16bit::PC => return self.program_counter
        }
    }

    pub fn set_AF(&mut self, value : u16){
        self.set_register(Registers8bit::A, (value >> 8) as u8);
        self.set_register(Registers8bit::F, value as u8);
    }

    pub fn set_BC(&mut self, value : u16){
        self.set_register(Registers8bit::B, (value >> 8) as u8);
        self.set_register(Registers8bit::C, value as u8);
    }

    pub fn set_DE(&mut self, value : u16){
        self.set_register(Registers8bit::D, (value >> 8) as u8);
        self.set_register(Registers8bit::E, value as u8);
    }

    pub fn set_HL(&mut self, value : u16){
        self.set_register(Registers8bit::H, (value >> 8) as u8);
        self.set_register(Registers8bit::L, value as u8);
    }

    pub fn set_SP(&mut self, value : u16){
        self.stack_pointer = value;
    }

    pub fn set_PC(&mut self, value : u16){
        self.program_counter = value;
    }

    pub fn set_16bit_register(&mut self, register : Registers16bit, value : u16) {
        match register {
            Registers16bit::AF => self.set_AF(value),
            Registers16bit::BC => self.set_BC(value),
            Registers16bit::DE => self.set_DE(value),
            Registers16bit::HL => self.set_HL(value),
            Registers16bit::SP => self.set_SP(value),
            Registers16bit::PC => self.set_PC(value,)  
        }
    }

}

