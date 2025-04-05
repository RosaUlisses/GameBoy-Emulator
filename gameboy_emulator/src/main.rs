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
    let mut emulator = Emulator::default();

    emulator.init_rom(Path::new("./tests/blargg-test-roms/cpu_instrs/individual/03-op sp,hl.gb"));
    
    print!("Oi");
}