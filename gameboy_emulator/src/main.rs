use cpu::CPU;

pub mod dispatch;
pub mod cpu;
pub mod bitwise;
pub mod instructions;
pub mod table;

use std::io;
use std::fs;

fn main() {
    let mut cpu = CPU::new();
    let mut buf = String::new();
    let mut serial_output = String::new();
    const REG_NAMES: [&str; 8] = [   
        "A", "F", "B", "C", "D", "E", "H", "L",
    ];

    // Load ROM into the first 0xFF bytes of memory
    const ROM_FILENAME: &str = "./tests/cpu_instrs/individual/01-special.gb";
    let rom_bytes = fs::read(ROM_FILENAME)
        .expect("Failed to read ROM");
    
    for i in 0..(rom_bytes.len()) {
        cpu.memory[i] = rom_bytes[i];
    }
    cpu.program_counter = 0x100;


    // Loop and execute instructions
    let mut until: i32 = -1;
    loop {
        for i in 0..1024 {
            cpu.execute_instruction();
        }
        
        // blarggs test - serial output
        if cpu.memory[0xFF02] == 0x81 {
            let c = cpu.memory[0xFF02] as char;
            serial_output.push(c);
            cpu.memory[0xFF02] = 0;
        }
        
        println!("Program Counter: 0x{:04X}", cpu.program_counter);
        println!("Stack Counter: 0x{:04X}", cpu.stack_pointer);
        print!("Registers: ");
        for i in 0..8 {
            print!("{}: 0x{:02X}, ", REG_NAMES[i], cpu.registers[i]);
        }
        println!();
        println!("MSG: {}", serial_output);
        
        if until < 0 || cpu.get_pc() == until as u16 {
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

