use crate::control_proccess_unity::CPU;
use crate::control_proccess_unity::Flags;

pub fn add(cpu : &mut CPU, destination : &mut u8, source : u8){
    let sum : u16 = (*destination as u16) + (source as u16);
    *destination += source;

    if *destination == 0 {
        cpu.set_flag(Flags::Z);
    } 
    cpu.reset_flag(Flags::N);
    if (*destination & 0x0F) + (source & 0x0F) >= 0x10 {
        cpu.set_flag(Flags::H);
    } 
   
    if sum == (*destination as u16) {
        cpu.set_flag(Flags::C);
    }
}


