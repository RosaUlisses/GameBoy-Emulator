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
use std::process::Command;

const roms_folder_path: &str = "blarggs_test_roms/cpu_instrs/individual";
const log_folder_path: &str = "gbdoctor/truth/unzipped/cpu_instrs";

fn main() {
    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/03-op sp,hl.gb", roms_folder_path)).as_str()));
    emulator.start_game_boy_doctor();
}