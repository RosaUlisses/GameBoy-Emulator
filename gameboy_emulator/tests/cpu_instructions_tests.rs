use std::{path::Path, fs, fmt::format, process::Command};
use gameboy_emulator::emulator::Emulator; 

const roms_folder_path: &str = "blarggs_test_roms/cpu_instrs/individual";
const log_folder_path: &str = "gbdoctor/truth/unzipped/cpu_instrs";

fn get_gameboy_doctor_output(rom_number: u8) -> String {
    let gameboy_doctor_output = Command::new("python3")
    .arg("./gbdoctor/gameboy-doctor.py")
    .arg("logs.txt")
    .arg("cpu_instrs")
    .arg(rom_number.to_string())
    .output()
    .expect("IT IS NOT POSSIBLE TO RUN THE COMMAND")
    .stdout;
   
    return String::from_utf8(gameboy_doctor_output).expect("ERROR, THE BYTES ARE INVALID");
}

#[test]
fn test3_op_sp_hl() {
    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/03-op sp,hl.gb", roms_folder_path)).as_str()));
    emulator.start_game_boy_doctor(); 

    let gameboy_doctor_output = get_gameboy_doctor_output(3);

    println!("test: 03-op sp, hl");
    println!("{}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test4_op_r_imm() {
    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/04-op r,imm.gb", roms_folder_path)).as_str()));
    emulator.start_game_boy_doctor(); 

    let gameboy_doctor_output = get_gameboy_doctor_output(4);

    println!("test: 04-op r, imm");
    println!("{}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test5_op_rp() {
    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/05-op rp.gb", roms_folder_path)).as_str()));
    emulator.start_game_boy_doctor(); 

    let gameboy_doctor_output = get_gameboy_doctor_output(5);

    println!("test: 05-op rp");
    println!("{}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test6_ld_r_r() {
    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/06-ld r,r.gb", roms_folder_path)).as_str()));
    emulator.start_game_boy_doctor(); 

    let gameboy_doctor_output = get_gameboy_doctor_output(6);

    println!("test: 06-ld r,r");
    println!("{}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test7_jr_jp_call_ret_rst() {
    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/07-jr,jp,call,ret,rst.gb", roms_folder_path)).as_str()));
    emulator.start_game_boy_doctor(); 

    let gameboy_doctor_output = get_gameboy_doctor_output(7);

    println!("test: 06-ld r,r");
    println!("{}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test8_misc_intrs() {
    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/08-misc instrs.gb", roms_folder_path)).as_str()));
    emulator.start_game_boy_doctor(); 

    let gameboy_doctor_output = get_gameboy_doctor_output(8);

    println!("test: 06-misc instrs");
    println!("{}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test9_op_r_r() {
    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/09-op r,r.gb", roms_folder_path)).as_str()));
    emulator.start_game_boy_doctor(); 

    let gameboy_doctor_output = get_gameboy_doctor_output(9);

    println!("test: 09-op r,r");
    println!("{}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test10_bit_ops() {
    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/10-bit ops.gb", roms_folder_path)).as_str()));
    emulator.start_game_boy_doctor(); 

    let gameboy_doctor_output = get_gameboy_doctor_output(10);

    println!("test: 10-bit ops");
    println!("{}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));
}

#[test]
fn test11_op_a_hl() {
    let mut emulator = Emulator::new_emulator_for_tests();
    emulator.init(Path::new(String::from(format!("{}/11-op a,(hl).gb", roms_folder_path)).as_str()));
    emulator.start_game_boy_doctor(); 

    let gameboy_doctor_output = get_gameboy_doctor_output(10);

    println!("test: 11-op a,(hl)");
    println!("{}", gameboy_doctor_output);
    assert!(gameboy_doctor_output.contains("SUCCESS"));

}