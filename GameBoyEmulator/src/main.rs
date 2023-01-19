mod control_process_unity;
use control_process_unity::CPU;
use control_process_unity::flags;

fn main() {

    let mut c = CPU::new();
    c.set_flag(flags::C);
    print!("oi");
}

