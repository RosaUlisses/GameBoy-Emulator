// use std::{path::Path, fs, fmt::format, process::Command};
use std::{path::Path, process::Command};
use gameboy_emulator::emulator::Emulator; 

const ROMS_FOLDER_PATH: &str = "tests/cpu_instrs/individual";
// const LOG_FOLDER_PATH: &str = "gbdoctor/truth/unzipped/cpu_instrs";

fn get_gameboy_doctor_output(rom_number: u8) -> String {
    let gameboy_doctor_output = Command::new("python3")
    .current_dir("./gbdoctor/")
    .arg("./gameboy-doctor")
    .arg(format!("../log{}.txt", rom_number))
    .arg("cpu_instrs")
    .arg(rom_number.to_string())
    .output()
    .expect("IT IS NOT POSSIBLE TO RUN THE COMMAND");
   
    let stdout_str = String::from_utf8(gameboy_doctor_output.stdout)
        .expect("ERROR, THE BYTES ARE INVALID");
    let stderr_str = String::from_utf8(gameboy_doctor_output.stderr)
        .expect("ERROR, THE BYTES ARE INVALID");

    println!("TESTING STDERR: {}", stderr_str);
    return stdout_str;
}

#[test]
fn test3_op_sp_hl() {
    let rom_number = 3;

    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/03-op sp,hl.gb", ROMS_FOLDER_PATH)).as_str()));
    emulator.start_game_boy_doctor(&format!("./log{}.txt", rom_number));

    println!("GENERATED LOG FILE");

    let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

    println!("test: 03-op sp, hl");
    println!("STDOUT: {}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test4_op_r_imm() {
    let rom_number = 4;

    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/04-op r,imm.gb", ROMS_FOLDER_PATH)).as_str()));
    emulator.start_game_boy_doctor(&format!("./log{}.txt", rom_number));

    println!("GENERATED LOG FILE");

    let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

    println!("test: 04-op r, imm");
    println!("STDOUT: {}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test5_op_rp() {
    let rom_number = 5;

    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/05-op rp.gb", ROMS_FOLDER_PATH)).as_str()));
    emulator.start_game_boy_doctor(&format!("./log{}.txt", rom_number));

    println!("GENERATED LOG FILE");

    let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

    println!("test: 05-op rp");
    println!("STDOUT: {}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test6_ld_r_r() {
    let rom_number = 6;

    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/06-ld r,r.gb", ROMS_FOLDER_PATH)).as_str()));
    emulator.start_game_boy_doctor(&format!("./log{}.txt", rom_number));

    println!("GENERATED LOG FILE");

    let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

    println!("test: 06-ld r,r");
    println!("STDOUT: {}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test7_jr_jp_call_ret_rst() {
    let rom_number = 7;

    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/07-jr,jp,call,ret,rst.gb", ROMS_FOLDER_PATH)).as_str()));
    emulator.start_game_boy_doctor(&format!("./log{}.txt", rom_number));

    println!("GENERATED LOG FILE");

    let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

    println!("test: 06-ld r,r");
    println!("STDOUT: {}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test8_misc_intrs() {
    let rom_number = 8;

    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/08-misc instrs.gb", ROMS_FOLDER_PATH)).as_str()));
    emulator.start_game_boy_doctor(&format!("./log{}.txt", rom_number));

    println!("GENERATED LOG FILE");

    let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

    println!("test: 06-misc instrs");
    println!("STDOUT: {}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test9_op_r_r() {
    let rom_number = 9;

    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/09-op r,r.gb", ROMS_FOLDER_PATH)).as_str()));
    emulator.start_game_boy_doctor(&format!("./log{}.txt", rom_number));

    println!("GENERATED LOG FILE");

    let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

    println!("test: 09-op r,r");
    println!("STDOUT: {}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test10_bit_ops() {
    let rom_number = 10;

    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/10-bit ops.gb", ROMS_FOLDER_PATH)).as_str()));
    emulator.start_game_boy_doctor(&format!("./log{}.txt", rom_number));

    println!("GENERATED LOG FILE");

    let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

    println!("test: 10-bit ops");
    println!("STDOUT: {}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test11_op_a_hl() {
    let rom_number = 11;

    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/11-op a,(hl).gb", ROMS_FOLDER_PATH)).as_str()));
    emulator.start_game_boy_doctor(&format!("./log{}.txt", rom_number));

    println!("GENERATED LOG FILE");

    let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

    println!("test: 11-op a,(hl)");
    println!("STDOUT: {}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));

}