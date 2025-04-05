// TODO -> REFATORAR AS CHAMADAS DE INIT_EMULATOR
// TODO -> CRIAR UM MODULO PARA O GB DOCTOR (run e getoutput, output correto)
// TODO -> CRIAR FUNCAO DE LOG no terminal



mod cpu_instructions_tests {
    use std::{path::Path, process::Command};
    use std::fs::File;
    use std::io::Write;
    use gameboy_emulator::cpu::Register8bit;
    use gameboy_emulator::emulator::Emulator;

    const ROMS_FOLDER_PATH: &str = "resources/blargg-test-roms/cpu_instrs/individual";

    fn get_gameboy_doctor_output(rom_number: u8) -> String {
        let gameboy_doctor_output = Command::new("python")
            .current_dir("resources/gameboy-doctor")
            .arg("gameboy-doctor")
            .arg(format!("logs/log{rom_number}.txt"))
            .arg("cpu_instrs")
            .arg(rom_number.to_string())
            .output()
            .expect("Unable to run the command");

        let stdout_str = String::from_utf8(gameboy_doctor_output.stdout)
            .expect("Error, invalid bytes");
        let stderr_str = String::from_utf8(gameboy_doctor_output.stderr)
            .expect("Error, invalid bytes");

        println!("Testing stderr: {}", stderr_str);
        
        stdout_str
    }

    fn init_emulator(rom_path: &Path) -> Emulator {
        let mut emulator = Emulator::default();

        emulator.cpu.set_register_8bit(Register8bit::A, 0x01);
        emulator.cpu.set_register_8bit(Register8bit::F, 0xB0);
        emulator.cpu.set_register_8bit(Register8bit::B, 0x00);
        emulator.cpu.set_register_8bit(Register8bit::C, 0x13);
        emulator.cpu.set_register_8bit(Register8bit::D, 0x00);
        emulator.cpu.set_register_8bit(Register8bit::E, 0xD8);
        emulator.cpu.set_register_8bit(Register8bit::H, 0x01);
        emulator.cpu.set_register_8bit(Register8bit::L, 0x4D);
        emulator.cpu.set_sp(0xFFFE);
        emulator.cpu.set_pc(0x0100);
        emulator.cpu.set_memory_8bit(0xFF44, 0x90);

        emulator.init_rom(rom_path);

        emulator
    }

    fn get_current_log(emulator: &mut Emulator) -> String {
        let mut log = String::new();
        const REG_NAMES: [&str; 8] = [
            "A", "F", "B", "C", "D", "E", "H", "L",
        ];

        for (i, reg_name) in REG_NAMES.iter().enumerate() {
            let reg_value = emulator.cpu.registers[i];
            log.push_str(format!("{reg_name}:{reg_value:02X} ").as_str());
        }
        log.push_str(format!("SP: {:04X} ", emulator.cpu.stack_pointer).as_str());
        log.push_str(format!("PC: {:04X} ", emulator.cpu.program_counter).as_str());

        log.push_str("PCMEM: ");

        let addr = emulator.cpu.get_pc();
        for i in 0..4 {
            let value = emulator.cpu.get_memory_8bit(addr.wrapping_add(i));
            log.push_str(format!("{:02X},", value).as_str());
        }

        log.pop();
        log.push('\n');

        log
    }


    pub fn start_game_boy_doctor(emulator: &mut Emulator, rom_number: u8) {
        let mut serial_output = String::new();
        let mut log_file = File::create(format!("resources/gameboy-doctor/logs/log{rom_number}.txt"))
            .expect("Error opening file");
        let mut log_string = String::new();

        loop {
            if serial_output.contains("Passed") || serial_output.contains("Failed") {
                break;
            }
            let max_runs = 1000;
            for _ in 0..max_runs {
                log_string.push_str(&get_current_log(emulator));
                emulator.cpu.execute_instruction();

                // blarggs test - serial output
                if emulator.cpu.memory[0xFF02] == 0x81 {
                    let c = emulator.cpu.memory[0xFF01] as char;
                    serial_output.push(c);
                    emulator.cpu.memory[0xFF02] = 0;
                }
            }
            log_file.write_all(log_string.as_bytes())
                .expect("Error writing to file");
            log_string.clear();
        }

        println!("Serial output: {serial_output}");
    }
    
    fn  print_test_output(test_case: String, gameboy_doctor_output: String) {
        println!("GENERATED LOG FILE");
        println!("{test_case}");
        println!("STDOUT: {gameboy_doctor_output}");
    }

    #[test]
    fn test3_op_sp_hl() {
        let rom_number = 3;
        let mut emulator = init_emulator(Path::new(&format!("{ROMS_FOLDER_PATH}/03-op sp,hl.gb")));
        start_game_boy_doctor(&mut emulator, rom_number);
        let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

        print_test_output(String::from("test: 03-op sp, hl"), gameboy_doctor_output.clone());
        assert!(gameboy_doctor_output.contains("SUCCESS"));
    }

    #[test]
    fn test4_op_r_imm() {
        let rom_number = 4;
        let mut emulator = init_emulator(Path::new(&format!("{ROMS_FOLDER_PATH}/04-op r,imm.gb")));
        start_game_boy_doctor(&mut emulator, rom_number);
        let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

        print_test_output(String::from("test: 04-op r, imm"), gameboy_doctor_output.clone());
        assert!(gameboy_doctor_output.contains("SUCCESS"));
    }

    #[test]
    fn test5_op_rp() {
        let rom_number = 5;
        let mut emulator = init_emulator(Path::new(&format!("{ROMS_FOLDER_PATH}/05-op rp.gb")));
        start_game_boy_doctor(&mut emulator, rom_number);
        let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

        print_test_output(String::from("test: 05-op rp"), gameboy_doctor_output.clone());
        assert!(gameboy_doctor_output.contains("SUCCESS"));
    }

    #[test]
    fn test6_ld_r_r() {
        let rom_number = 6;
        let mut emulator = init_emulator(Path::new(&format!("{ROMS_FOLDER_PATH}/06-ld r,r.gb")));
        start_game_boy_doctor(&mut emulator, rom_number);
        let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

        print_test_output(String::from("test: 06-ld r,r"), gameboy_doctor_output.clone());
        assert!(gameboy_doctor_output.contains("SUCCESS"));
    }

    #[test]
    fn test7_jr_jp_call_ret_rst() {
        let rom_number = 7;
        let mut emulator = init_emulator(Path::new(&format!("{ROMS_FOLDER_PATH}/07-jr,jp,call,ret,rst.gb")));
        start_game_boy_doctor(&mut emulator, rom_number);
        let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

        print_test_output(String::from("test: 07-ld r,r"), gameboy_doctor_output.clone());
        assert!(gameboy_doctor_output.contains("SUCCESS"));
    }

    #[test]
    fn test8_misc_intrs() {
        let rom_number = 8;
        let mut emulator = init_emulator(Path::new(&format!("{ROMS_FOLDER_PATH}/08-misc instrs.gb")));
        start_game_boy_doctor(&mut emulator, rom_number);
        let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

        print_test_output(String::from("test: 08-misc instrs"), gameboy_doctor_output.clone());
        assert!(gameboy_doctor_output.contains("SUCCESS"));
    }

    #[test]
    fn test9_op_r_r() {
        let rom_number = 9;
        let mut emulator = init_emulator(Path::new(&format!("{ROMS_FOLDER_PATH}/09-op r,r.gb")));
        start_game_boy_doctor(&mut emulator, rom_number);
        let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

        print_test_output(String::from("test: 09-op r,r"), gameboy_doctor_output.clone());
        assert!(gameboy_doctor_output.contains("SUCCESS"));
    }

    #[test]
    fn test10_bit_ops() {
        let rom_number = 10;
        let mut emulator = init_emulator(Path::new(&format!("{ROMS_FOLDER_PATH}/10-bit ops.gb")));
        start_game_boy_doctor(&mut emulator, rom_number);
        let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

        print_test_output(String::from("test: 10-bit ops"), gameboy_doctor_output.clone());
        assert!(gameboy_doctor_output.contains("SUCCESS"));
    }

    #[test]
    fn test11_op_a_hl() {
        let rom_number = 11;
        let mut emulator = init_emulator(Path::new(&format!("{ROMS_FOLDER_PATH}/11-op a,(hl).gb")));
        start_game_boy_doctor(&mut emulator, rom_number);
        let gameboy_doctor_output = get_gameboy_doctor_output(rom_number);

        print_test_output(String::from("test: 11-op a,(hl)"), gameboy_doctor_output.clone());
        assert!(gameboy_doctor_output.contains("SUCCESS"));
    }
}

