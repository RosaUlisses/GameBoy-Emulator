use crate::dispatch::Instruction;
use crate::dispatch::Instruction::*;
use crate::dispatch::AddressingMode8bit as Mode8;
use crate::dispatch::AddressingMode16bit as Mode16;
use crate::cpu::Register8bit as Reg8;
use crate::cpu::Register16bit as Reg16;

use crate::instructions::*;

pub const INSTRUCTIONS: [Instruction; 512] = [
    Implied     (    nop),
    Op16bit16bit(   ld16,   Mode16::Register(Reg16::BC),             Mode16::Immediate),
    Op8bit8bit  (     ld,    Mode8::Indirect(Reg16::BC),      Mode8::Register(Reg8::A)),
    Op16bit     (  inc16,   Mode16::Register(Reg16::BC)),
    Op8bit      (    inc,      Mode8::Register(Reg8::B)),
    Op8bit      (    dec,      Mode8::Register(Reg8::B)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::B),              Mode8::Immediate),
    Implied     (   rlca),
    Op16bit16bit(   ld16,               Mode16::Address,          Mode16::StackPointer),
    Op16bit     (  addhl,   Mode16::Register(Reg16::BC)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),    Mode8::Indirect(Reg16::BC)),
    Op16bit     (  dec16,   Mode16::Register(Reg16::BC)),
    Op8bit      (    inc,      Mode8::Register(Reg8::C)),
    Op8bit      (    dec,      Mode8::Register(Reg8::C)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::C),              Mode8::Immediate),
    Implied     (   rrca),
    Op8bit      (   stop,              Mode8::Immediate),
    Op16bit16bit(   ld16,   Mode16::Register(Reg16::DE),             Mode16::Immediate),
    Op8bit8bit  (     ld,    Mode8::Indirect(Reg16::DE),      Mode8::Register(Reg8::A)),
    Op16bit     (  inc16,   Mode16::Register(Reg16::DE)),
    Op8bit      (    inc,      Mode8::Register(Reg8::D)),
    Op8bit      (    dec,      Mode8::Register(Reg8::D)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::D),              Mode8::Immediate),
    Implied     (    rla),
    Op8bit      (     jr,              Mode8::Immediate),
    Op16bit     (  addhl,   Mode16::Register(Reg16::DE)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),    Mode8::Indirect(Reg16::DE)),
    Op16bit     (  dec16,   Mode16::Register(Reg16::DE)),
    Op8bit      (    inc,      Mode8::Register(Reg8::E)),
    Op8bit      (    dec,      Mode8::Register(Reg8::E)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::E),              Mode8::Immediate),
    Implied     (    rra),
    Op8bit      (   jrnz,              Mode8::Immediate),
    Op16bit16bit(   ld16,   Mode16::Register(Reg16::HL),             Mode16::Immediate),
    Op8bit8bit  (    ldi,    Mode8::Indirect(Reg16::HL),      Mode8::Register(Reg8::A)),
    Op16bit     (  inc16,   Mode16::Register(Reg16::HL)),
    Op8bit      (    inc,      Mode8::Register(Reg8::H)),
    Op8bit      (    dec,      Mode8::Register(Reg8::H)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::H),              Mode8::Immediate),
    Implied     (    daa),
    Op8bit      (    jrz,              Mode8::Immediate),
    Op16bit     (  addhl,   Mode16::Register(Reg16::HL)),
    Op8bit8bit  (    ldi,      Mode8::Register(Reg8::A),    Mode8::Indirect(Reg16::HL)),
    Op16bit     (  dec16,   Mode16::Register(Reg16::HL)),
    Op8bit      (    inc,      Mode8::Register(Reg8::L)),
    Op8bit      (    dec,      Mode8::Register(Reg8::L)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::L),              Mode8::Immediate),
    Implied     (    cpl),
    Op8bit      (   jrnc,              Mode8::Immediate),
    Op16bit16bit(   ld16,          Mode16::StackPointer,             Mode16::Immediate),
    Op8bit8bit  (    ldd,    Mode8::Indirect(Reg16::HL),      Mode8::Register(Reg8::A)),
    Op16bit     (  inc16,          Mode16::StackPointer),
    Op8bit      (    inc,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    dec,    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (     ld,    Mode8::Indirect(Reg16::HL),              Mode8::Immediate),
    Implied     (    scf),
    Op8bit      (    jrc,              Mode8::Immediate),
    Op16bit     (  addhl,          Mode16::StackPointer),
    Op8bit8bit  (    ldd,      Mode8::Register(Reg8::A),    Mode8::Indirect(Reg16::HL)),
    Op16bit     (  dec16,          Mode16::StackPointer),
    Op8bit      (    inc,      Mode8::Register(Reg8::A)),
    Op8bit      (    dec,      Mode8::Register(Reg8::A)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),              Mode8::Immediate),
    Implied     (    ccf),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::B),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::B),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::B),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::B),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::B),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::B),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::B),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::B),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::C),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::C),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::C),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::C),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::C),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::C),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::C),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::C),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::D),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::D),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::D),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::D),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::D),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::D),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::D),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::D),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::E),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::E),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::E),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::E),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::E),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::E),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::E),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::E),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::H),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::H),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::H),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::H),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::H),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::H),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::H),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::H),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::L),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::L),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::L),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::L),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::L),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::L),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::L),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::L),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (     ld,    Mode8::Indirect(Reg16::HL),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (     ld,    Mode8::Indirect(Reg16::HL),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (     ld,    Mode8::Indirect(Reg16::HL),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (     ld,    Mode8::Indirect(Reg16::HL),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (     ld,    Mode8::Indirect(Reg16::HL),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (     ld,    Mode8::Indirect(Reg16::HL),      Mode8::Register(Reg8::L)),
    Implied     (   halt),
    Op8bit8bit  (     ld,    Mode8::Indirect(Reg16::HL),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),      Mode8::Register(Reg8::A)),
    Op8bit      (    add,      Mode8::Register(Reg8::B)),
    Op8bit      (    add,      Mode8::Register(Reg8::C)),
    Op8bit      (    add,      Mode8::Register(Reg8::D)),
    Op8bit      (    add,      Mode8::Register(Reg8::E)),
    Op8bit      (    add,      Mode8::Register(Reg8::H)),
    Op8bit      (    add,      Mode8::Register(Reg8::L)),
    Op8bit      (    add,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    add,      Mode8::Register(Reg8::A)),
    Op8bit      (    adc,      Mode8::Register(Reg8::B)),
    Op8bit      (    adc,      Mode8::Register(Reg8::C)),
    Op8bit      (    adc,      Mode8::Register(Reg8::D)),
    Op8bit      (    adc,      Mode8::Register(Reg8::E)),
    Op8bit      (    adc,      Mode8::Register(Reg8::H)),
    Op8bit      (    adc,      Mode8::Register(Reg8::L)),
    Op8bit      (    adc,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    adc,      Mode8::Register(Reg8::A)),
    Op8bit      (    sub,      Mode8::Register(Reg8::B)),
    Op8bit      (    sub,      Mode8::Register(Reg8::C)),
    Op8bit      (    sub,      Mode8::Register(Reg8::D)),
    Op8bit      (    sub,      Mode8::Register(Reg8::E)),
    Op8bit      (    sub,      Mode8::Register(Reg8::H)),
    Op8bit      (    sub,      Mode8::Register(Reg8::L)),
    Op8bit      (    sub,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    sub,      Mode8::Register(Reg8::A)),
    Op8bit      (    sbc,      Mode8::Register(Reg8::B)),
    Op8bit      (    sbc,      Mode8::Register(Reg8::C)),
    Op8bit      (    sbc,      Mode8::Register(Reg8::D)),
    Op8bit      (    sbc,      Mode8::Register(Reg8::E)),
    Op8bit      (    sbc,      Mode8::Register(Reg8::H)),
    Op8bit      (    sbc,      Mode8::Register(Reg8::L)),
    Op8bit      (    sbc,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    sbc,      Mode8::Register(Reg8::A)),
    Op8bit      (    and,      Mode8::Register(Reg8::B)),
    Op8bit      (    and,      Mode8::Register(Reg8::C)),
    Op8bit      (    and,      Mode8::Register(Reg8::D)),
    Op8bit      (    and,      Mode8::Register(Reg8::E)),
    Op8bit      (    and,      Mode8::Register(Reg8::H)),
    Op8bit      (    and,      Mode8::Register(Reg8::L)),
    Op8bit      (    and,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    and,      Mode8::Register(Reg8::A)),
    Op8bit      (    xor,      Mode8::Register(Reg8::B)),
    Op8bit      (    xor,      Mode8::Register(Reg8::C)),
    Op8bit      (    xor,      Mode8::Register(Reg8::D)),
    Op8bit      (    xor,      Mode8::Register(Reg8::E)),
    Op8bit      (    xor,      Mode8::Register(Reg8::H)),
    Op8bit      (    xor,      Mode8::Register(Reg8::L)),
    Op8bit      (    xor,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    xor,      Mode8::Register(Reg8::A)),
    Op8bit      (     or,      Mode8::Register(Reg8::B)),
    Op8bit      (     or,      Mode8::Register(Reg8::C)),
    Op8bit      (     or,      Mode8::Register(Reg8::D)),
    Op8bit      (     or,      Mode8::Register(Reg8::E)),
    Op8bit      (     or,      Mode8::Register(Reg8::H)),
    Op8bit      (     or,      Mode8::Register(Reg8::L)),
    Op8bit      (     or,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (     or,      Mode8::Register(Reg8::A)),
    Op8bit      (     cp,      Mode8::Register(Reg8::B)),
    Op8bit      (     cp,      Mode8::Register(Reg8::C)),
    Op8bit      (     cp,      Mode8::Register(Reg8::D)),
    Op8bit      (     cp,      Mode8::Register(Reg8::E)),
    Op8bit      (     cp,      Mode8::Register(Reg8::H)),
    Op8bit      (     cp,      Mode8::Register(Reg8::L)),
    Op8bit      (     cp,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (     cp,      Mode8::Register(Reg8::A)),
    Implied     (  retnz),
    Op16bit     (    pop,   Mode16::Register(Reg16::BC)),
    Op16bit     (   jpnz,             Mode16::Immediate),
    Op16bit     (     jp,             Mode16::Immediate),
    Op16bit     ( callnz,             Mode16::Immediate),
    Op16bit     (   push,   Mode16::Register(Reg16::BC)),
    Op8bit      (    add,              Mode8::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0000)),
    Implied     (   retz),
    Implied     (    ret),
    Op16bit     (    jpz,             Mode16::Immediate),
    Prefix      ,
    Op16bit     (  callz,             Mode16::Immediate),
    Op16bit     (   call,             Mode16::Immediate),
    Op8bit      (    adc,              Mode8::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0008)),
    Implied     (  retnc),
    Op16bit     (    pop,   Mode16::Register(Reg16::DE)),
    Op16bit     (   jpnc,             Mode16::Immediate),
    Invalid,
    Op16bit     ( callnc,             Mode16::Immediate),
    Op16bit     (   push,   Mode16::Register(Reg16::DE)),
    Op8bit      (    sub,              Mode8::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0010)),
    Implied     (   retc),
    Implied     (   reti),
    Op16bit     (    jpc,             Mode16::Immediate),
    Invalid,
    Op16bit     (  callc,             Mode16::Immediate),
    Invalid,
    Op8bit      (    sbc,              Mode8::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0018)),
    Op8bit8bit  (     ld,             Mode8::IndexedImm,      Mode8::Register(Reg8::A)),
    Op16bit     (    pop,   Mode16::Register(Reg16::HL)),
    Op8bit8bit  (     ld,               Mode8::IndexedC,      Mode8::Register(Reg8::A)),
    Invalid,
    Invalid,
    Op16bit     (   push,   Mode16::Register(Reg16::HL)),
    Op8bit      (    and,              Mode8::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0020)),
    Op8bit      (  addsp,              Mode8::Immediate),
    Op16bit     (     jp,   Mode16::Register(Reg16::HL)),
    Op8bit8bit  (     ld,                Mode8::Address,      Mode8::Register(Reg8::A)),
    Invalid,
    Invalid,
    Invalid,
    Op8bit      (    xor,              Mode8::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0028)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),             Mode8::IndexedImm),
    Op16bit     (    pop,   Mode16::Register(Reg16::AF)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),               Mode8::IndexedC),
    Implied     (     di),
    Invalid,
    Op16bit     (   push,   Mode16::Register(Reg16::AF)),
    Op8bit      (     or,              Mode8::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0030)),
    Op8bit      (   ldhl,              Mode8::Immediate),
    Op16bit16bit(   ld16,          Mode16::StackPointer,   Mode16::Register(Reg16::HL)),
    Op8bit8bit  (     ld,      Mode8::Register(Reg8::A),                Mode8::Address),
    Implied     (     ei),
    Invalid,
    Invalid,
    Op8bit      (     cp,              Mode8::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0038)),
    Op8bit      (    rlc,      Mode8::Register(Reg8::B)),
    Op8bit      (    rlc,      Mode8::Register(Reg8::C)),
    Op8bit      (    rlc,      Mode8::Register(Reg8::D)),
    Op8bit      (    rlc,      Mode8::Register(Reg8::E)),
    Op8bit      (    rlc,      Mode8::Register(Reg8::H)),
    Op8bit      (    rlc,      Mode8::Register(Reg8::L)),
    Op8bit      (    rlc,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    rlc,      Mode8::Register(Reg8::A)),
    Op8bit      (    rrc,      Mode8::Register(Reg8::B)),
    Op8bit      (    rrc,      Mode8::Register(Reg8::C)),
    Op8bit      (    rrc,      Mode8::Register(Reg8::D)),
    Op8bit      (    rrc,      Mode8::Register(Reg8::E)),
    Op8bit      (    rrc,      Mode8::Register(Reg8::H)),
    Op8bit      (    rrc,      Mode8::Register(Reg8::L)),
    Op8bit      (    rrc,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    rrc,      Mode8::Register(Reg8::A)),
    Op8bit      (     rl,      Mode8::Register(Reg8::B)),
    Op8bit      (     rl,      Mode8::Register(Reg8::C)),
    Op8bit      (     rl,      Mode8::Register(Reg8::D)),
    Op8bit      (     rl,      Mode8::Register(Reg8::E)),
    Op8bit      (     rl,      Mode8::Register(Reg8::H)),
    Op8bit      (     rl,      Mode8::Register(Reg8::L)),
    Op8bit      (     rl,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (     rl,      Mode8::Register(Reg8::A)),
    Op8bit      (     rr,      Mode8::Register(Reg8::B)),
    Op8bit      (     rr,      Mode8::Register(Reg8::C)),
    Op8bit      (     rr,      Mode8::Register(Reg8::D)),
    Op8bit      (     rr,      Mode8::Register(Reg8::E)),
    Op8bit      (     rr,      Mode8::Register(Reg8::H)),
    Op8bit      (     rr,      Mode8::Register(Reg8::L)),
    Op8bit      (     rr,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (     rr,      Mode8::Register(Reg8::A)),
    Op8bit      (    sla,      Mode8::Register(Reg8::B)),
    Op8bit      (    sla,      Mode8::Register(Reg8::C)),
    Op8bit      (    sla,      Mode8::Register(Reg8::D)),
    Op8bit      (    sla,      Mode8::Register(Reg8::E)),
    Op8bit      (    sla,      Mode8::Register(Reg8::H)),
    Op8bit      (    sla,      Mode8::Register(Reg8::L)),
    Op8bit      (    sla,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    sla,      Mode8::Register(Reg8::A)),
    Op8bit      (    sra,      Mode8::Register(Reg8::B)),
    Op8bit      (    sra,      Mode8::Register(Reg8::C)),
    Op8bit      (    sra,      Mode8::Register(Reg8::D)),
    Op8bit      (    sra,      Mode8::Register(Reg8::E)),
    Op8bit      (    sra,      Mode8::Register(Reg8::H)),
    Op8bit      (    sra,      Mode8::Register(Reg8::L)),
    Op8bit      (    sra,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    sra,      Mode8::Register(Reg8::A)),
    Op8bit      (   swap,      Mode8::Register(Reg8::B)),
    Op8bit      (   swap,      Mode8::Register(Reg8::C)),
    Op8bit      (   swap,      Mode8::Register(Reg8::D)),
    Op8bit      (   swap,      Mode8::Register(Reg8::E)),
    Op8bit      (   swap,      Mode8::Register(Reg8::H)),
    Op8bit      (   swap,      Mode8::Register(Reg8::L)),
    Op8bit      (   swap,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (   swap,      Mode8::Register(Reg8::A)),
    Op8bit      (    srl,      Mode8::Register(Reg8::B)),
    Op8bit      (    srl,      Mode8::Register(Reg8::C)),
    Op8bit      (    srl,      Mode8::Register(Reg8::D)),
    Op8bit      (    srl,      Mode8::Register(Reg8::E)),
    Op8bit      (    srl,      Mode8::Register(Reg8::H)),
    Op8bit      (    srl,      Mode8::Register(Reg8::L)),
    Op8bit      (    srl,    Mode8::Indirect(Reg16::HL)),
    Op8bit      (    srl,      Mode8::Register(Reg8::A)),

    Op8bit8bit  (    bit,               Mode8::Fixed(0),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    bit,               Mode8::Fixed(0),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    bit,               Mode8::Fixed(0),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    bit,               Mode8::Fixed(0),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    bit,               Mode8::Fixed(0),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    bit,               Mode8::Fixed(0),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    bit,               Mode8::Fixed(0),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    bit,               Mode8::Fixed(0),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    bit,               Mode8::Fixed(1),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    bit,               Mode8::Fixed(1),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    bit,               Mode8::Fixed(1),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    bit,               Mode8::Fixed(1),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    bit,               Mode8::Fixed(1),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    bit,               Mode8::Fixed(1),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    bit,               Mode8::Fixed(1),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    bit,               Mode8::Fixed(1),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    bit,               Mode8::Fixed(2),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    bit,               Mode8::Fixed(2),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    bit,               Mode8::Fixed(2),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    bit,               Mode8::Fixed(2),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    bit,               Mode8::Fixed(2),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    bit,               Mode8::Fixed(2),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    bit,               Mode8::Fixed(2),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    bit,               Mode8::Fixed(2),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    bit,               Mode8::Fixed(3),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    bit,               Mode8::Fixed(3),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    bit,               Mode8::Fixed(3),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    bit,               Mode8::Fixed(3),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    bit,               Mode8::Fixed(3),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    bit,               Mode8::Fixed(3),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    bit,               Mode8::Fixed(3),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    bit,               Mode8::Fixed(3),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    bit,               Mode8::Fixed(4),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    bit,               Mode8::Fixed(4),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    bit,               Mode8::Fixed(4),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    bit,               Mode8::Fixed(4),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    bit,               Mode8::Fixed(4),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    bit,               Mode8::Fixed(4),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    bit,               Mode8::Fixed(4),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    bit,               Mode8::Fixed(4),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    bit,               Mode8::Fixed(5),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    bit,               Mode8::Fixed(5),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    bit,               Mode8::Fixed(5),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    bit,               Mode8::Fixed(5),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    bit,               Mode8::Fixed(5),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    bit,               Mode8::Fixed(5),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    bit,               Mode8::Fixed(5),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    bit,               Mode8::Fixed(5),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    bit,               Mode8::Fixed(6),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    bit,               Mode8::Fixed(6),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    bit,               Mode8::Fixed(6),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    bit,               Mode8::Fixed(6),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    bit,               Mode8::Fixed(6),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    bit,               Mode8::Fixed(6),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    bit,               Mode8::Fixed(6),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    bit,               Mode8::Fixed(6),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    bit,               Mode8::Fixed(7),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    bit,               Mode8::Fixed(7),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    bit,               Mode8::Fixed(7),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    bit,               Mode8::Fixed(7),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    bit,               Mode8::Fixed(7),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    bit,               Mode8::Fixed(7),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    bit,               Mode8::Fixed(7),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    bit,               Mode8::Fixed(7),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    res,               Mode8::Fixed(0),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    res,               Mode8::Fixed(0),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    res,               Mode8::Fixed(0),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    res,               Mode8::Fixed(0),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    res,               Mode8::Fixed(0),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    res,               Mode8::Fixed(0),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    res,               Mode8::Fixed(0),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    res,               Mode8::Fixed(0),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    res,               Mode8::Fixed(1),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    res,               Mode8::Fixed(1),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    res,               Mode8::Fixed(1),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    res,               Mode8::Fixed(1),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    res,               Mode8::Fixed(1),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    res,               Mode8::Fixed(1),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    res,               Mode8::Fixed(1),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    res,               Mode8::Fixed(1),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    res,               Mode8::Fixed(2),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    res,               Mode8::Fixed(2),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    res,               Mode8::Fixed(2),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    res,               Mode8::Fixed(2),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    res,               Mode8::Fixed(2),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    res,               Mode8::Fixed(2),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    res,               Mode8::Fixed(2),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    res,               Mode8::Fixed(2),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    res,               Mode8::Fixed(3),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    res,               Mode8::Fixed(3),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    res,               Mode8::Fixed(3),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    res,               Mode8::Fixed(3),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    res,               Mode8::Fixed(3),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    res,               Mode8::Fixed(3),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    res,               Mode8::Fixed(3),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    res,               Mode8::Fixed(3),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    res,               Mode8::Fixed(4),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    res,               Mode8::Fixed(4),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    res,               Mode8::Fixed(4),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    res,               Mode8::Fixed(4),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    res,               Mode8::Fixed(4),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    res,               Mode8::Fixed(4),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    res,               Mode8::Fixed(4),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    res,               Mode8::Fixed(4),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    res,               Mode8::Fixed(5),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    res,               Mode8::Fixed(5),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    res,               Mode8::Fixed(5),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    res,               Mode8::Fixed(5),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    res,               Mode8::Fixed(5),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    res,               Mode8::Fixed(5),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    res,               Mode8::Fixed(5),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    res,               Mode8::Fixed(5),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    res,               Mode8::Fixed(6),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    res,               Mode8::Fixed(6),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    res,               Mode8::Fixed(6),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    res,               Mode8::Fixed(6),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    res,               Mode8::Fixed(6),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    res,               Mode8::Fixed(6),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    res,               Mode8::Fixed(6),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    res,               Mode8::Fixed(6),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    res,               Mode8::Fixed(7),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    res,               Mode8::Fixed(7),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    res,               Mode8::Fixed(7),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    res,               Mode8::Fixed(7),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    res,               Mode8::Fixed(7),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    res,               Mode8::Fixed(7),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    res,               Mode8::Fixed(7),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    res,               Mode8::Fixed(7),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    set,               Mode8::Fixed(0),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    set,               Mode8::Fixed(0),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    set,               Mode8::Fixed(0),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    set,               Mode8::Fixed(0),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    set,               Mode8::Fixed(0),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    set,               Mode8::Fixed(0),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    set,               Mode8::Fixed(0),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    set,               Mode8::Fixed(0),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    set,               Mode8::Fixed(1),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    set,               Mode8::Fixed(1),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    set,               Mode8::Fixed(1),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    set,               Mode8::Fixed(1),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    set,               Mode8::Fixed(1),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    set,               Mode8::Fixed(1),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    set,               Mode8::Fixed(1),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    set,               Mode8::Fixed(1),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    set,               Mode8::Fixed(2),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    set,               Mode8::Fixed(2),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    set,               Mode8::Fixed(2),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    set,               Mode8::Fixed(2),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    set,               Mode8::Fixed(2),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    set,               Mode8::Fixed(2),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    set,               Mode8::Fixed(2),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    set,               Mode8::Fixed(2),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    set,               Mode8::Fixed(3),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    set,               Mode8::Fixed(3),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    set,               Mode8::Fixed(3),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    set,               Mode8::Fixed(3),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    set,               Mode8::Fixed(3),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    set,               Mode8::Fixed(3),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    set,               Mode8::Fixed(3),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    set,               Mode8::Fixed(3),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    set,               Mode8::Fixed(4),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    set,               Mode8::Fixed(4),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    set,               Mode8::Fixed(4),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    set,               Mode8::Fixed(4),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    set,               Mode8::Fixed(4),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    set,               Mode8::Fixed(4),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    set,               Mode8::Fixed(4),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    set,               Mode8::Fixed(4),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    set,               Mode8::Fixed(5),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    set,               Mode8::Fixed(5),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    set,               Mode8::Fixed(5),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    set,               Mode8::Fixed(5),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    set,               Mode8::Fixed(5),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    set,               Mode8::Fixed(5),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    set,               Mode8::Fixed(5),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    set,               Mode8::Fixed(5),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    set,               Mode8::Fixed(6),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    set,               Mode8::Fixed(6),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    set,               Mode8::Fixed(6),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    set,               Mode8::Fixed(6),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    set,               Mode8::Fixed(6),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    set,               Mode8::Fixed(6),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    set,               Mode8::Fixed(6),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    set,               Mode8::Fixed(6),      Mode8::Register(Reg8::A)),
    Op8bit8bit  (    set,               Mode8::Fixed(7),      Mode8::Register(Reg8::B)),
    Op8bit8bit  (    set,               Mode8::Fixed(7),      Mode8::Register(Reg8::C)),
    Op8bit8bit  (    set,               Mode8::Fixed(7),      Mode8::Register(Reg8::D)),
    Op8bit8bit  (    set,               Mode8::Fixed(7),      Mode8::Register(Reg8::E)),
    Op8bit8bit  (    set,               Mode8::Fixed(7),      Mode8::Register(Reg8::H)),
    Op8bit8bit  (    set,               Mode8::Fixed(7),      Mode8::Register(Reg8::L)),
    Op8bit8bit  (    set,               Mode8::Fixed(7),    Mode8::Indirect(Reg16::HL)),
    Op8bit8bit  (    set,               Mode8::Fixed(7),      Mode8::Register(Reg8::A)),
];