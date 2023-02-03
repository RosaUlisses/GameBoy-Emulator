pub mod emulator;
use emulator::Emulator;
pub mod dispatch;
pub mod cpu;
pub mod bitwise;
pub mod instructions;
pub mod table;

use std::io;
use std::path::Path;
use std::fs;

const PATH: &str = "tests/cpu_instrs/individual/10-bit ops.gb";

fn main() {
    let rom_path = String::from(PATH);
    let mut emulator = Emulator::new();
    emulator.init(Path::new(&rom_path));
    emulator.init_gameboy_doctor();
    emulator.start_game_boy_doctor();
}