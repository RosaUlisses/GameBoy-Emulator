use crate::bitwise;
use crate::table::INSTRUCTIONS;

const REGISTER_COUNT: usize = 8;
const MEMORY_SIZE: usize = 65536;
const IE_REGISTER_ADDRESS: usize = 0xFFFF;
// const IRQ_REGISTER_ADDRESS: usize = 0xFF0F;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    Z = 7,
    N = 6,
    H = 5,
    C = 4,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Register8bit {
    A = 0,
    F = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    H = 6,
    L = 7,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Register16bit {
    AF = 0,
    BC = 1,
    DE = 2,
    HL = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {
    VBLank,
    LCDStatus,
    Timer,
    Serial,
    Joypad,
}

pub struct CPU {
    pub registers: [u8; REGISTER_COUNT],
    pub stack_pointer: u16,
    pub program_counter: u16,
    pub memory: [u8; MEMORY_SIZE],
    pub ime_flag: bool,
}

impl CPU {
    pub fn new() -> Box<Self> {
        Box::new(CPU {
            registers: [0x00; 8],
            stack_pointer: 0xFFFF,
            program_counter: 0x0000,
            memory: [0x00; MEMORY_SIZE],
            ime_flag: false,
        })
    }

    pub fn load_rom(&mut self, rom_bytes: &[u8]) {
        let until = rom_bytes.len();
        self.memory[..until].copy_from_slice(rom_bytes);
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.fetch_next_8bits_pc();
        INSTRUCTIONS[opcode as usize].execute(self);
    }

    pub fn handle_interrupts(&mut self) {
        if self.interrupts_enabled() {
            match self.active_interrupt() {
                Interrupt::VBLank => self.handle_vblank_interrupt(),
                Interrupt::LCDStatus => self.handle_lcdstatus_interrupt(),
                Interrupt::Timer => self.handle_timer_interrupt(),
                Interrupt::Serial => self.handle_serial_interrupt(),
                Interrupt::Joypad => self.handle_joypad_interrupt(),
            }
        }
    }

    pub fn get_memory_8bit(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn set_memory_8bit(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn get_memory_16bit(&self, address: u16) -> u16 {
        bitwise::hl_to_16b(
            self.get_memory_8bit(address),
            self.get_memory_8bit(address.wrapping_add(1)),
        )
    }

    pub fn set_memory_16bit(&mut self, address: u16, value: u16) {
        self.set_memory_8bit(address, bitwise::get_low(value));
        self.set_memory_8bit(address.wrapping_add(1), bitwise::get_high(value));
    }

    pub fn get_8bit_memory_from_sp(&mut self) -> u8 {
        self.get_memory_8bit(self.stack_pointer)
    }

    pub fn set_8bit_memory_from_sp(&mut self, value: u8) {
        self.set_memory_8bit(self.stack_pointer, value);
    }

    pub fn get_16bit_memory_from_sp(&mut self) -> u16 {
        self.get_memory_16bit(self.stack_pointer)
    }

    pub fn set_16bit_memory_from_sp(&mut self, value: u16) {
        self.set_memory_16bit(self.stack_pointer, value);
    }

    pub fn get_8bit_memory_from_pc(&mut self) -> u8 {
        self.get_memory_8bit(self.program_counter)
    }

    pub fn set_8bit_memory_from_pc(&mut self, value: u8) {
        self.set_memory_8bit(self.program_counter, value);
    }

    pub fn get_16bit_memory_from_pc(&mut self) -> u16 {
        self.get_memory_16bit(self.program_counter)
    }

    pub fn set_16bit_memory_from_pc(&mut self, value: u16) {
        self.set_memory_16bit(self.program_counter, value);
    }

    pub fn fetch_next_8bits_pc(&mut self) -> u8 {
        let value = self.get_8bit_memory_from_pc();
        self.program_counter = self.program_counter.wrapping_add(1);
        value
    }

    pub fn fetch_next_16bits_pc(&mut self) -> u16 {
        let value = self.get_16bit_memory_from_pc();
        self.program_counter = self.program_counter.wrapping_add(2);
        value
    }

    pub fn push_8bit_sp(&mut self, value: u8) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.set_8bit_memory_from_sp(value);
    }

    pub fn pop_8bit_sp(&mut self) -> u8 {
        let value = self.get_8bit_memory_from_sp();
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        value
    }

    pub fn push_16bit_sp(&mut self, value: u16) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(2);
        self.set_16bit_memory_from_sp(value);
    }

    pub fn pop_16bit_sp(&mut self) -> u16 {
        let value = self.get_16bit_memory_from_sp();
        self.stack_pointer = self.stack_pointer.wrapping_add(2);
        value
    }

    pub fn get_8bit_memory_from_register(&mut self, register: Register16bit) -> u8 {
        let address = self.get_register_16bit(register);
        self.get_memory_8bit(address)
    }

    pub fn set_8bit_memory_from_register(&mut self, value: u8, register: Register16bit) {
        let address = self.get_register_16bit(register);
        self.set_memory_8bit(address, value);
    }

    pub fn get_16bit_memory_from_register(&mut self, register: Register16bit) -> u16 {
        let address = self.get_register_16bit(register);
        self.get_memory_16bit(address)
    }

    pub fn set_16bit_memory_from_register(&mut self, value: u16, register: Register16bit) {
        let address = self.get_register_16bit(register);
        self.set_memory_16bit(address, value);
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        bitwise::get_bit(self.get_register_8bit(Register8bit::F), flag as usize)
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        bitwise::assign_bit(
            &mut self.registers[Register8bit::F as usize],
            flag as usize,
            value,
        );
    }

    pub fn get_register_8bit(&self, register: Register8bit) -> u8 {
        self.registers[register as usize]
    }

    pub fn set_register_8bit(&mut self, register: Register8bit, value: u8) {
        self.registers[register as usize] = value;
        if register == Register8bit::F {
            self.registers[register as usize] &= 0xF0;
        }
    }

    pub fn get_register_16bit(&self, register: Register16bit) -> u16 {
        let index: usize = register as usize * 2;
        bitwise::hl_to_16b(self.registers[index + 1], self.registers[index])
    }

    pub fn set_register_16bit(&mut self, register: Register16bit, value: u16) {
        let index = register as usize * 2;

        self.registers[index] = bitwise::get_high(value);
        self.registers[index + 1] = bitwise::get_low(value);
        if register == Register16bit::AF {
            self.registers[index + 1] &= 0xF0;
        }
    }

    pub fn get_sp(&self) -> u16 {
        self.stack_pointer
    }

    pub fn set_sp(&mut self, value: u16) {
        self.stack_pointer = value;
    }

    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    pub fn set_pc(&mut self, value: u16) {
        self.program_counter = value;
    }

    pub fn set_ime_flag(&mut self) {
        self.ime_flag = true;
    }

    pub fn reset_ime_flag(&mut self) {
        self.ime_flag = false;
    }

    fn interrupts_enabled(&self) -> bool {
        self.memory[IE_REGISTER_ADDRESS] & (0xF) != 0
    }

    fn active_interrupt(&self) -> Interrupt {
        if (self.memory[IE_REGISTER_ADDRESS] & 1) != 0 {
            Interrupt::VBLank
        } else if (self.memory[IE_REGISTER_ADDRESS] & 2) != 0 {
            Interrupt::LCDStatus
        } else if (self.memory[IE_REGISTER_ADDRESS] & 4) != 0 {
            Interrupt::Timer
        } else if (self.memory[IE_REGISTER_ADDRESS] & 8) != 0 {
            Interrupt::Serial
        } else {
            Interrupt::Joypad
        }
    }

    fn handle_vblank_interrupt(&mut self) {
        // TODO -> implement vblank interrupt
        todo!("VBLANK INTERRUPT");
    }

    fn handle_lcdstatus_interrupt(&mut self) {
        // TODO -> implement lcd status interrupt
        todo!("LCD STATUS INTERRUPT");
    }

    fn handle_timer_interrupt(&mut self) {
        // TODO -> implement timer interrupt
        todo!("TIMER INTERRUPT");
    }

    fn handle_serial_interrupt(&mut self) {
        // TODO -> implement serial interrupt
        todo!("SERIAL INTERRUPT");
    }

    fn handle_joypad_interrupt(&mut self) {
        // TODO -> implement joypad interrupt
        todo!("JOYPAD INTERRUPT");
    }
}
