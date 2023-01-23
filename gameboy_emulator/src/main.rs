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
    instructions::push(&mut cpu, Operand16bit::Immediate(0x1234));
    instructions::ld(&mut cpu,
        Operand8bit::Register(cpu::Registers8bit::A),
        Operand8bit::Immediate(0xFE));
    instructions::cp(&mut cpu, Operand8bit::Immediate(0xFF));

    if cpu.get_flag(Flags::Z) {
        println!("Equal!");
    }
    else {
        println!("Not equal!");
    }
}

