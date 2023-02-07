use crate::cpu::CPU;
use crate::cpu::Registers8bit;
use std::f32::consts::E;
use std::io;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct Emulator {
    cpu: CPU
}

impl Emulator {
    pub fn new() -> Self {
        return Emulator {cpu: CPU::new()};
    }

    pub fn new_emulator_for_tests() -> Self {
        let mut emulator = Emulator::new();
        emulator.init_gameboy_doctor();
        return emulator;
    }

    pub fn init_gameboy_doctor(&mut self) {
        self.cpu.set_register_8bit(Registers8bit::A, 0x01);
        self.cpu.set_register_8bit(Registers8bit::F, 0xB0);
        self.cpu.set_register_8bit(Registers8bit::B, 0x00);
        self.cpu.set_register_8bit(Registers8bit::C, 0x13);
        self.cpu.set_register_8bit(Registers8bit::D, 0x00);
        self.cpu.set_register_8bit(Registers8bit::E, 0xD8);
        self.cpu.set_register_8bit(Registers8bit::H, 0x01);
        self.cpu.set_register_8bit(Registers8bit::L, 0x4D);
        self.cpu.set_sp(0xFFFE);
        self.cpu.set_pc(0x100);
        self.cpu.set_memory_8bit(0xFF44, 0x90);
    }

    pub fn init(&mut self, rom_path: &Path) {
        let rom_bytes = fs::read(rom_path)
            .expect("ERROR, IT IS NOT POSSIBLE TO READ THE ROM"); 
        self.init_gameboy_doctor();
        self.cpu.load_rom(&rom_bytes);
        
        // let rom_bytes = fs::read("ROMS/bootstrap.gb")
        //     .expect("ERROR, IT IS NOT POSSIBLE TO READ THE ROM"); 
        // self.cpu.load_rom(&rom_bytes);
    }

    fn get_current_log(&self) -> String {
        let mut log = String::new(); 
        const REG_NAMES: [&str; 8] = [   
            "A", "F", "B", "C", "D", "E", "H", "L",
        ];

        for i in 0..8 {
            log.push_str(format!("{}:{:02X} ", REG_NAMES[i], self.cpu.registers[i]).as_str());
        }
        log.push_str(format!("SP:{:04X} ", self.cpu.stack_pointer).as_str());
        log.push_str(format!("PC:{:04X} ", self.cpu.program_counter).as_str());

        log.push_str("PCMEM:");

        let addr = self.cpu.get_pc();
        for i in 0..4 {
            let value = self.cpu.get_memory_8bit(addr.wrapping_add(i));
            log.push_str(format!("{:02X},", value).as_str());
        }
        
        log.pop();
        log.push_str("\n");

        return log;
    }

    pub fn start_game_boy_doctor(&mut self) {
        let mut serial_output = String::new();
        let mut log_file = File::create("logs.txt")
            .expect("ERROR OPENING FILE");
        let mut log_string = String::new();

        loop {
            if serial_output.contains("Passed") || serial_output.contains("Failed"){
                return;
            }
            const MAX_RUNS: usize = 1000;
            for _ in 0..MAX_RUNS {
                log_string.push_str(&self.get_current_log());
                self.cpu.execute_instruction();

                // blarggs test - serial output
                if self.cpu.memory[0xFF02] == 0x81 {
                    let c = self.cpu.memory[0xFF01] as char;
                    serial_output.push(c);
                    self.cpu.memory[0xFF02] = 0;
                }
            }

            log_file.write_all(log_string.as_bytes())
                .expect("Error writing to file");
            log_string.clear();
        }
    }

}
