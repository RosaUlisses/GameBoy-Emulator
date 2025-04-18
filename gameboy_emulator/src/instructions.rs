use crate::bitwise;
use crate::cpu::Flag;
use crate::cpu::Register16bit;
use crate::cpu::Register8bit;
use crate::cpu::CPU;
use crate::dispatch::Operand16bit;
use crate::dispatch::Operand8bit;

pub fn ld(cpu: &mut CPU, operand1: Operand8bit, operand2: Operand8bit) {
    let value = operand2.get(cpu);
    operand1.set(cpu, value);
}

pub fn ldi(cpu: &mut CPU, operand1: Operand8bit, operand2: Operand8bit) {
    ld(cpu, operand1, operand2);
    inc16(cpu, Operand16bit::Register(Register16bit::HL));
}

pub fn ldd(cpu: &mut CPU, operand1: Operand8bit, operand2: Operand8bit) {
    ld(cpu, operand1, operand2);
    dec16(cpu, Operand16bit::Register(Register16bit::HL));
}

pub fn ld16(cpu: &mut CPU, operand1: Operand16bit, operand2: Operand16bit) {
    let value = operand2.get(cpu);
    operand1.set(cpu, value);
}

pub fn ldhl(cpu: &mut CPU, operand1: Operand8bit) {
    let value1 = operand1.get(cpu) as i8 as u16;
    let value2 = cpu.get_sp();
    let result = value2.wrapping_add(value1);

    cpu.set_flag(Flag::Z, false);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, (value1 & 0x0F) + (value2 & 0x0F) >= 0x10);
    cpu.set_flag(Flag::C, (value1 & 0xFF) + (value2 & 0xFF) >= 0x100);

    cpu.set_register_16bit(Register16bit::HL, result);
}

pub fn push(cpu: &mut CPU, operand1: Operand16bit) {
    let value = operand1.get(cpu);
    cpu.push_16bit_sp(value);
}

pub fn pop(cpu: &mut CPU, operand1: Operand16bit) {
    let value = cpu.pop_16bit_sp();
    operand1.set(cpu, value);
}

pub fn add(cpu: &mut CPU, operand1: Operand8bit) {
    let value1 = cpu.get_register_8bit(Register8bit::A);
    let value2 = operand1.get(cpu);

    let (sum, overflow) = value1.overflowing_add(value2);
    cpu.set_register_8bit(Register8bit::A, sum);

    cpu.set_flag(Flag::Z, sum == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, (value1 & 0x0F) + (value2 & 0x0F) >= 0x10);
    cpu.set_flag(Flag::C, overflow);
}

pub fn adc(cpu: &mut CPU, operand1: Operand8bit) {
    let value1 = cpu.get_register_8bit(Register8bit::A);
    let value2 = operand1.get(cpu);

    let carry = cpu.get_flag(Flag::C) as u8; // 1 or 0

    let (sum, overflow1) = value1.overflowing_add(value2);
    let (sum, overflow2) = sum.overflowing_add(carry);
    cpu.set_register_8bit(Register8bit::A, sum);

    cpu.set_flag(Flag::Z, sum == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, (value1 & 0x0F) + (value2 & 0x0F) + carry >= 0x10);
    cpu.set_flag(Flag::C, overflow1 || overflow2);
}

pub fn sub(cpu: &mut CPU, operand1: Operand8bit) {
    let value1 = cpu.get_register_8bit(Register8bit::A);
    let value2 = operand1.get(cpu);

    let (diff, underflow) = value1.overflowing_sub(value2);
    cpu.set_register_8bit(Register8bit::A, diff);

    cpu.set_flag(Flag::Z, diff == 0);
    cpu.set_flag(Flag::N, true);
    cpu.set_flag(Flag::H, (value1 & 0x0F) < (value2 & 0x0F));
    cpu.set_flag(Flag::C, underflow);
}

pub fn sbc(cpu: &mut CPU, operand1: Operand8bit) {
    let value1 = cpu.get_register_8bit(Register8bit::A);
    let value2 = operand1.get(cpu);

    let carry = cpu.get_flag(Flag::C) as u8; // 1 or 0

    let halfunderflow1 = (value1 & 0x0F) < (value2 & 0x0F);
    let (diff, underflow1) = value1.overflowing_sub(value2);
    let halfunderflow2 = (diff & 0x0F) < carry;
    let (diff, underflow2) = diff.overflowing_sub(carry);
    cpu.set_register_8bit(Register8bit::A, diff);

    cpu.set_flag(Flag::Z, diff == 0);
    cpu.set_flag(Flag::N, true);
    cpu.set_flag(Flag::H, halfunderflow1 || halfunderflow2);
    cpu.set_flag(Flag::C, underflow1 || underflow2);
}

pub fn and(cpu: &mut CPU, operand1: Operand8bit) {
    let result = cpu.get_register_8bit(Register8bit::A) & operand1.get(cpu);

    cpu.set_register_8bit(Register8bit::A, result);

    cpu.set_flag(Flag::Z, result == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, true);
    cpu.set_flag(Flag::C, false);
}

pub fn or(cpu: &mut CPU, operand1: Operand8bit) {
    let result = cpu.get_register_8bit(Register8bit::A) | operand1.get(cpu);

    cpu.set_register_8bit(Register8bit::A, result);

    cpu.set_flag(Flag::Z, result == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, false);
}

pub fn xor(cpu: &mut CPU, operand1: Operand8bit) {
    let result = cpu.get_register_8bit(Register8bit::A) ^ operand1.get(cpu);

    cpu.set_register_8bit(Register8bit::A, result);

    cpu.set_flag(Flag::Z, result == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, false);
}

pub fn cp(cpu: &mut CPU, operand1: Operand8bit) {
    let value1: u8 = cpu.get_register_8bit(Register8bit::A);
    let value2: u8 = operand1.get(cpu);

    cpu.set_flag(Flag::Z, value1 == value2);
    cpu.set_flag(Flag::N, true);
    cpu.set_flag(Flag::H, (value1 & 0x0F) < (value2 & 0x0F));
    cpu.set_flag(Flag::C, value1 < value2);
}

pub fn inc(cpu: &mut CPU, operand1: Operand8bit) {
    let incremented: u8 = operand1.get(cpu).wrapping_add(1);

    operand1.set(cpu, incremented);

    cpu.set_flag(Flag::Z, incremented == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, (incremented & 0x0F) == 0);
}

pub fn dec(cpu: &mut CPU, operand1: Operand8bit) {
    let decremented: u8 = operand1.get(cpu).wrapping_sub(1);

    operand1.set(cpu, decremented);

    cpu.set_flag(Flag::Z, decremented == 0);
    cpu.set_flag(Flag::N, true);
    cpu.set_flag(Flag::H, (decremented & 0x0F) == 0xF);
}

pub fn addhl(cpu: &mut CPU, operand1: Operand16bit) {
    let value1 = cpu.get_register_16bit(Register16bit::HL);
    let value2 = operand1.get(cpu);

    let (sum, overflow) = value1.overflowing_add(value2);
    cpu.set_register_16bit(Register16bit::HL, sum);

    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, (value1 & 0x0FFF) + (value2 & 0x0FFF) >= 0x1000);
    cpu.set_flag(Flag::C, overflow);
}

pub fn addsp(cpu: &mut CPU, operand1: Operand8bit) {
    let value1 = cpu.get_sp();
    let value2 = (operand1.get(cpu) as i8) as u16;

    let sum = value1.wrapping_add(value2);
    cpu.set_sp(sum);

    cpu.set_flag(Flag::Z, false);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, (value1 & 0x0F) + (value2 & 0x0F) >= 0x10);
    cpu.set_flag(Flag::C, (value1 & 0xFF) + (value2 & 0xFF) >= 0x100);
}

pub fn inc16(cpu: &mut CPU, operand1: Operand16bit) {
    let value = operand1.get(cpu).wrapping_add(1);
    operand1.set(cpu, value)
}

pub fn dec16(cpu: &mut CPU, operand1: Operand16bit) {
    let value = operand1.get(cpu).wrapping_sub(1);
    operand1.set(cpu, value)
}

pub fn swap(cpu: &mut CPU, operand1: Operand8bit) {
    let value = (operand1.get(cpu) << 4) | (operand1.get(cpu) >> 4);
    operand1.set(cpu, value);

    cpu.set_flag(Flag::Z, value == 0);

    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, false);
}

pub fn daa(cpu: &mut CPU) {
    let mut a_value = cpu.get_register_8bit(Register8bit::A);

    if !cpu.get_flag(Flag::N) {
        if cpu.get_flag(Flag::C) || (a_value > 0x99) {
            a_value = a_value.wrapping_add(0x60);
            cpu.set_flag(Flag::C, true);
        }
        if cpu.get_flag(Flag::H) || ((a_value & 0x0f) > 0x09) {
            a_value = a_value.wrapping_add(0x06);
        }
    } else {
        if cpu.get_flag(Flag::C) {
            a_value = a_value.wrapping_sub(0x60);
        }
        if cpu.get_flag(Flag::H) {
            a_value = a_value.wrapping_sub(0x06);
        }
    }

    cpu.set_register_8bit(Register8bit::A, a_value);
    cpu.set_flag(Flag::Z, a_value == 0);
    cpu.set_flag(Flag::H, false);
}

pub fn cpl(cpu: &mut CPU) {
    let value = cpu.get_register_8bit(Register8bit::A);
    cpu.set_register_8bit(Register8bit::A, !value);

    cpu.set_flag(Flag::N, true);
    cpu.set_flag(Flag::H, true);
}

pub fn ccf(cpu: &mut CPU) {
    cpu.set_flag(Flag::C, !cpu.get_flag(Flag::C));

    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
}

pub fn scf(cpu: &mut CPU) {
    cpu.set_flag(Flag::C, true);

    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
}

pub fn rlca(cpu: &mut CPU) {
    let value = cpu.get_register_8bit(Register8bit::A);
    let result = value.rotate_left(1);

    cpu.set_register_8bit(Register8bit::A, result);

    cpu.set_flag(Flag::Z, false);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, (result & 1) != 0);
}

pub fn rla(cpu: &mut CPU) {
    let value = cpu.get_register_8bit(Register8bit::A);
    let tmp = value.wrapping_shl(1);
    let result = tmp | (cpu.get_flag(Flag::C) as u8);

    cpu.set_register_8bit(Register8bit::A, result);

    cpu.set_flag(Flag::Z, false);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, value & 0x80 != 0);
}

pub fn rrca(cpu: &mut CPU) {
    let value = cpu.get_register_8bit(Register8bit::A);
    let result = value.rotate_right(1);

    cpu.set_register_8bit(Register8bit::A, result);

    cpu.set_flag(Flag::Z, false);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, (value & 1) != 0);
}

pub fn rra(cpu: &mut CPU) {
    let value = cpu.get_register_8bit(Register8bit::A);
    let tmp = value.wrapping_shr(1);
    let carry = value & 1 != 0;
    let result = tmp | ((cpu.get_flag(Flag::C) as u8) << 7);

    cpu.set_register_8bit(Register8bit::A, result);

    cpu.set_flag(Flag::Z, false);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, carry);
}

pub fn rlc(cpu: &mut CPU, operand1: Operand8bit) {
    let value = operand1.get(cpu);
    let result = value.rotate_left(1);

    operand1.set(cpu, result);

    cpu.set_flag(Flag::Z, result == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, (result & 1) != 0);
}

pub fn rl(cpu: &mut CPU, operand1: Operand8bit) {
    let value = operand1.get(cpu);

    let tmp = value.wrapping_shl(1);
    let result = tmp | (cpu.get_flag(Flag::C) as u8);

    operand1.set(cpu, result);

    cpu.set_flag(Flag::Z, result == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, value & 0x80 != 0);
}

pub fn rrc(cpu: &mut CPU, operand1: Operand8bit) {
    let value = operand1.get(cpu);
    let result = value.rotate_right(1);

    operand1.set(cpu, result);

    cpu.set_flag(Flag::Z, result == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, (value & 1) != 0);
}

pub fn rr(cpu: &mut CPU, operand1: Operand8bit) {
    let value = operand1.get(cpu);
    let tmp = value.wrapping_shr(1);
    let carry = value & 1 != 0;
    let result = tmp | ((cpu.get_flag(Flag::C) as u8) << 7);

    operand1.set(cpu, result);

    cpu.set_flag(Flag::Z, result == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, carry);
}

pub fn sla(cpu: &mut CPU, operand1: Operand8bit) {
    let value = operand1.get(cpu);
    let result = value.wrapping_shl(1);

    operand1.set(cpu, result);

    cpu.set_flag(Flag::Z, result == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, value & 0x80 != 0);
}

pub fn sra(cpu: &mut CPU, operand1: Operand8bit) {
    let value = operand1.get(cpu);
    let tmp = value.wrapping_shr(1);
    let result = tmp | (value & 0x80);

    operand1.set(cpu, result);

    cpu.set_flag(Flag::Z, result == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, value & 1 != 0);
}

pub fn srl(cpu: &mut CPU, operand1: Operand8bit) {
    let value = operand1.get(cpu);
    let result = value.wrapping_shr(1);

    operand1.set(cpu, result);

    cpu.set_flag(Flag::Z, result == 0);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, false);
    cpu.set_flag(Flag::C, value & 1 != 0);
}

pub fn bit(cpu: &mut CPU, operand1: Operand8bit, operand2: Operand8bit) {
    let bit = operand1.get(cpu);
    let bit = bitwise::get_bit(operand2.get(cpu), bit as usize);

    cpu.set_flag(Flag::Z, !bit);
    cpu.set_flag(Flag::N, false);
    cpu.set_flag(Flag::H, true);
}

pub fn set(cpu: &mut CPU, operand1: Operand8bit, operand2: Operand8bit) {
    let bit = operand1.get(cpu);
    let value = operand2.get(cpu);
    operand2.set(cpu, bitwise::set_bit(value, bit as usize, true));
}

pub fn res(cpu: &mut CPU, operand1: Operand8bit, operand2: Operand8bit) {
    let bit = operand1.get(cpu);
    let value = operand2.get(cpu);
    operand2.set(cpu, bitwise::set_bit(value, bit as usize, false));
}

pub fn jp(cpu: &mut CPU, operand1: Operand16bit) {
    let value = operand1.get(cpu);
    cpu.set_pc(value);
}

pub fn jpnz(cpu: &mut CPU, operand1: Operand16bit) {
    if !cpu.get_flag(Flag::Z) {
        jp(cpu, operand1);
    }
}
pub fn jpz(cpu: &mut CPU, operand1: Operand16bit) {
    if cpu.get_flag(Flag::Z) {
        jp(cpu, operand1);
    }
}

pub fn jpnc(cpu: &mut CPU, operand1: Operand16bit) {
    if !cpu.get_flag(Flag::C) {
        jp(cpu, operand1);
    }
}

pub fn jpc(cpu: &mut CPU, operand1: Operand16bit) {
    if cpu.get_flag(Flag::C) {
        jp(cpu, operand1);
    }
}

pub fn jphl(cpu: &mut CPU) {
    cpu.set_pc(cpu.get_register_16bit(Register16bit::HL));
}

pub fn jr(cpu: &mut CPU, operand1: Operand8bit) {
    let value = operand1.get(cpu) as i8 as u16;
    let result = cpu.get_pc().wrapping_add(value);
    cpu.set_pc(result);
}

pub fn jrnz(cpu: &mut CPU, operand1: Operand8bit) {
    if !cpu.get_flag(Flag::Z) {
        jr(cpu, operand1);
    }
}

pub fn jrz(cpu: &mut CPU, operand1: Operand8bit) {
    if cpu.get_flag(Flag::Z) {
        jr(cpu, operand1);
    }
}

pub fn jrnc(cpu: &mut CPU, operand1: Operand8bit) {
    if !cpu.get_flag(Flag::C) {
        jr(cpu, operand1);
    }
}

pub fn jrc(cpu: &mut CPU, operand1: Operand8bit) {
    if cpu.get_flag(Flag::C) {
        jr(cpu, operand1);
    }
}

pub fn call(cpu: &mut CPU, operand1: Operand16bit) {
    cpu.push_16bit_sp(cpu.get_pc());
    jp(cpu, operand1);
}

pub fn callnz(cpu: &mut CPU, operand1: Operand16bit) {
    if !cpu.get_flag(Flag::Z) {
        call(cpu, operand1);
    }
}
pub fn callz(cpu: &mut CPU, operand1: Operand16bit) {
    if cpu.get_flag(Flag::Z) {
        call(cpu, operand1);
    }
}

pub fn callnc(cpu: &mut CPU, operand1: Operand16bit) {
    if !cpu.get_flag(Flag::C) {
        call(cpu, operand1);
    }
}

pub fn callc(cpu: &mut CPU, operand1: Operand16bit) {
    if cpu.get_flag(Flag::C) {
        call(cpu, operand1);
    }
}

pub fn ret(cpu: &mut CPU) {
    let address = cpu.pop_16bit_sp();
    cpu.set_pc(address);
}

pub fn retnz(cpu: &mut CPU) {
    if !cpu.get_flag(Flag::Z) {
        ret(cpu);
    }
}
pub fn retz(cpu: &mut CPU) {
    if cpu.get_flag(Flag::Z) {
        ret(cpu);
    }
}

pub fn retnc(cpu: &mut CPU) {
    if !cpu.get_flag(Flag::C) {
        ret(cpu);
    }
}

pub fn retc(cpu: &mut CPU) {
    if cpu.get_flag(Flag::C) {
        ret(cpu);
    }
}

pub fn rst(cpu: &mut CPU, operand1: Operand16bit) {
    call(cpu, operand1);
}

pub fn ei(cpu: &mut CPU) {
    cpu.set_ime_flag();
    println!("Interrupts enabled!");
}

pub fn di(cpu: &mut CPU) {
    cpu.reset_ime_flag();
    println!("Interrupts disabled!");
}

pub fn nop(_: &mut CPU) {}

pub fn stop(_cpu: &mut CPU, _: Operand8bit) {
    print!("Stop intruction");
    //todo!("Stop instruction");
}

pub fn halt(_cpu: &mut CPU) {
    todo!("Halt instruction");
}

pub fn reti(cpu: &mut CPU) {
    ei(cpu);
    ret(cpu);
}
