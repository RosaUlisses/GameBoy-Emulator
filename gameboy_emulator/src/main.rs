pub mod actions;

pub mod cpu;
use actions::Operand16bit;
use actions::Operand8bit;
use cpu::CPU;
use cpu::Flags;

fn main() {
    
    let mut cpu: CPU = CPU::new();
    actions::push(&mut cpu, Operand16bit::Immediate(0x1234));
    actions::ld(&mut cpu,
        Operand8bit::Register(cpu::Registers8bit::A),
        Operand8bit::Immediate(0xFE));
    actions::cp(&mut cpu, Operand8bit::Immediate(0xFF));

    if cpu.get_flag(Flags::Z) {
        println!("Equal!");
    }
    else {
        println!("Not equal!");
    }
}

