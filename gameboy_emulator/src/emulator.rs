use crate::cpu::CPU;
use std::fs;
use std::path::Path;

pub struct Emulator {
    pub cpu: Box<CPU>,
}

impl Default for Emulator {
    fn default() -> Self {
        Emulator { cpu: CPU::new() }
    }
}

impl Emulator {
    pub fn init_rom(&mut self, rom_path: &Path) {
        let rom_bytes = fs::read(rom_path).expect("Error reading rom");
        self.cpu.load_rom(&rom_bytes);
    }
}
