pub mod emulator;
// use emulator::Emulator;
pub mod dispatch;
pub mod cpu;
pub mod bitwise;
pub mod instructions;
pub mod table;


// use std::path::Path;

// const ROMS_FOLDER_PATH: &str = "tests/cpu_instrs/individual";
// const LOG_FOLDER_PATH: &str = "gbdoctor/truth/unzipped/cpu_instrs";

fn main() {
    // let mut emulator = Emulator::new_emulator_for_tests();
    
    // emulator.init(Path::new(String::from(
    //     format!("{}/03-op sp,hl.gb", ROMS_FOLDER_PATH)
    // ).as_str()));
    
    // emulator.start_game_boy_doctor("");
}