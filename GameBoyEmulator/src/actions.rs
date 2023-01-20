mod cpu;
use cpu::CPU;
use cpu::Flags;
use cpu::Registers;

fn Add(cpu : &mut CPU, register : Registers){
    cpu.registers[Registers::A as usize] += cpu.registers[register as usize];
}

