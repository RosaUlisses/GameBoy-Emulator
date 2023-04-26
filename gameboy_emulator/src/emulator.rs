use crate::cpu::CPU;
use crate::cpu::Registers8bit;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct Emulator {
    pub cpu: CPU,
}

impl Emulator {
    pub fn new() -> Self {
        return Emulator { cpu: CPU::new() };
    }

    pub fn init_rom(&mut self, rom_path: &Path) {
        let rom_bytes = fs::read(rom_path).expect("ERROR READING ROM");
        self.cpu.load_rom(&rom_bytes);
    }
}
