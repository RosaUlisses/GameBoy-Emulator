use crate::cpu::CPU;
use crate::cpu::Registers8bit;
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
    }

    pub fn init(&mut self, rom_path: &Path) {
        let rom_bytes = fs::read(rom_path).expect("ERROR, IT IS NOT POSSIBLE TO READ THE ROM"); 
        self.init_gameboy_doctor();
        self.cpu.load_rom(&rom_bytes);
    }

    fn get_current_log(&self) -> String {
        let mut log = String::new(); 
        const REG_NAMES: [&str; 8] = [   
            "A", "F", "B", "C", "D", "E", "H", "L",
        ];

        for i in 0..8 {
            log.push_str(format!("{}:{:X} ", REG_NAMES[i], self.cpu.registers[i]).as_str());
        }
        log.push_str(format!("SP:{:X} ", self.cpu.stack_pointer).as_str());
        log.push_str(format!("PC:{:X} ", self.cpu.program_counter).as_str());

        log.push_str("PCMEM:");

        let addr = self.cpu.get_pc();
        for i in 0..4 {
            let value = self.cpu.get_memory_8bit(addr.wrapping_add(i));
            log.push_str(format!("{:X},", value).as_str());
        }
        
        log.pop();
        log.push_str("\n");

        return log;
    }

    pub fn start_game_boy_doctor(&mut self) {

        let mut log_file = File::create("logs.txt")
            .expect("ERROR OPENING FILE");
        let mut log_string = String::new();

        for _ in 0..0x100000 {
            self.cpu.execute_instruction();
            log_string.push_str(&self.get_current_log());
        }
        
        log_file.write_all(log_string.as_bytes())
            .expect("Error writing to file");
    }

    pub fn start(&mut self) {
        let mut buf = String::new();
        let mut serial_output = String::new();
        const REG_NAMES: [&str; 8] = [   
            "A", "F", "B", "C", "D", "E", "H", "L",
        ];
        
        self.cpu.set_pc(0x100);    

        let mut until: i32 = -1;
        loop {
            for _ in 0..0x1000 {
                self.cpu.execute_instruction();
                // blarggs test - serial output
                if self.cpu.memory[0xFF02] == 0x81 {
                    let c = self.cpu.memory[0xFF01] as char;
                    serial_output.push(c);
                    self.cpu.memory[0xFF02] = 0;
                }
            }
            
            println!("Program Counter: 0x{:04X}", self.cpu.program_counter);
            println!("Stack Counter: 0x{:04X}", self.cpu.stack_pointer);
            print!("Registers: ");
            for i in 0..8 {
                print!("{}: 0x{:02X}, ", REG_NAMES[i], self.cpu.registers[i]);
            }
            println!();
            println!("MSG: {}", serial_output);
            
            if until < 0 || self.cpu.get_pc() == until as u16 {
                until = -1;
                io::stdin()
                    .read_line(&mut buf)
                    .expect("Reading line failed");
                if buf.starts_with("exit") {
                    break;
                }
                else if buf.starts_with("until") {
                    let num = i32::from_str_radix(&buf[5..].trim(), 16)
                        .expect("Error parsing int");
                    until = num;
                }
            }
            buf.clear();
        }
    }
}
