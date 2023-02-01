use crate::cpu::CPU;
use std::io;
use std::fs;

use std::path::Path;

pub struct Emulator {
    cpu: CPU
}

impl Emulator {
    pub fn new() -> Self {
        return Emulator {cpu: CPU::new()};
    }

    pub fn init(&mut self, rom_path: &Path) {
        let rom_bytes = fs::read(rom_path).expect("ERROR, IT IS NOT POSSIBLE TO READ THE ROM"); 
        self.cpu.load_rom(&rom_bytes);
    }

    pub fn start(&mut self) {
        // let mut buf = String::new();
        let mut serial_output = String::new();
        const REG_NAMES: [&str; 8] = [   
            "A", "F", "B", "C", "D", "E", "H", "L",
        ];
        
        self.cpu.set_pc(0x100);    

        // let mut until: i32 = -1;
        loop {
            for _ in 0..0x10000 {
                self.cpu.execute_instruction();
            }
            
            // blarggs test - serial output
            if self.cpu.memory[0xFF02] == 0x81 {
                let c = self.cpu.memory[0xFF01] as char;
                serial_output.push(c);
                self.cpu.memory[0xFF02] = 0;
            }
            
            println!("Program Counter: 0x{:04X}", self.cpu.program_counter);
            println!("Stack Counter: 0x{:04X}", self.cpu.stack_pointer);
            print!("Registers: ");
            for i in 0..8 {
                print!("{}: 0x{:02X}, ", REG_NAMES[i], self.cpu.registers[i]);
            }
            println!();
            println!("MSG: {}", serial_output);
            
            // if until < 0 || self.cpu.get_pc() == until as u16 {
            //     until = -1;
            //     io::stdin()
            //         .read_line(&mut buf)
            //         .expect("Reading line failed");
            //     if buf.starts_with("exit") {
            //         break;
            //     }
            //     else if buf.starts_with("until") {
            //         let num = i32::from_str_radix(&buf[5..].trim(), 16)
            //             .expect("Error parsing int");
            //         until = num;
            //     }
            // }
            // buf.clear();
        }
    }
}
