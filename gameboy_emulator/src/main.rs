pub mod dispatch;
pub mod cpu;
pub mod bitwise;
pub mod instructions;

use cpu::CPU;
use cpu::Flags;

use dispatch::Operand16bit;
use dispatch::Operand8bit;

fn main() {
    
    let mut cpu: CPU = CPU::new();
    loop {
        let opcode = cpu.fetch_next_8bits_pc();
        // Execute
        println!("${:04X}: ${:02X}", cpu.get_pc(), opcode);
    }
}

