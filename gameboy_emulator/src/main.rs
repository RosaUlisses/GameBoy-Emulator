pub mod emulator;
use emulator::Emulator;
pub mod dispatch;
pub mod cpu;
pub mod bitwise;
pub mod instructions;
pub mod table;

use std::path::Path;

const PATH: &str = "tests/cpu_instrs/individual/02-interrupts.gb";

fn main() {

    let rom_path = String::from(PATH);
    let mut emulator = Emulator::new();
    emulator.init(Path::new(&rom_path));
    emulator.init_gameboy_doctor();
    emulator.start_game_boy_doctor();
}