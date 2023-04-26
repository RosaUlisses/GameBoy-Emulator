use crate::emulator::Emulator;

pub mod emulator;
// use emulator::Emulator;
pub mod dispatch;
pub mod cpu;
pub mod bitwise;
pub mod instructions;
pub mod table;


use std::path::Path;


fn main() {
    let mut emulator = Emulator::new();
    
    emulator.init_rom(Path::new(String::from(
        format!("C:/Programming/GameBoy-Emulator/gameboy_emulator/tests/blargg-test-roms/cpu_instrs/individual/03-op sp,hl.gb")
    ).as_str()));
    
    print!("Oi");
}