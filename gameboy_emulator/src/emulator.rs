use crate::cpu::CPU;
use crate::table::INSTRUCTIONS;

use std::io;
use std::fs;

pub struct Emulator {
    cpu: CPU
}

impl Emulator {
    pub fn new() -> Self {
        return Emulator {cpu: CPU::new()};
    }

    pub fn init(&mut self, rom_path: String) {
        let rom_bytes = fs::read(rom_path).expect("ERROR, IT IS NOT POSSIBLE TO READ THE ROM"); 
        self.cpu.load_ROM(&rom_bytes);
    }
}
