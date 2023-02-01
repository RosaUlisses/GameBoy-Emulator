pub mod emulator;
use emulator::Emulator;
pub mod dispatch;
pub mod cpu;
pub mod bitwise;
pub mod instructions;
pub mod table;

use std::io;
use std::fs;

const PATH: &str = "C:\\Programação\\GameBoy-Emulator\\gameboy_emulator\\ROMS\\Batman.gb";

fn main() {
    let rom_path = String::from(PATH);
    let mut emulator = Emulator::new();
    emulator.init(rom_path);
}

