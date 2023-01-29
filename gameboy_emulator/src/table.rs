use crate::dispatch::Instruction;
use crate::dispatch::Instruction::*;
use crate::dispatch::AddressingMode8bit as Mode8;
use crate::dispatch::AddressingMode16bit as Mode16;
use crate::cpu::Registers8bit as Reg8;
use crate::cpu::Registers16bit as Reg16;

use crate::instructions::*;

// MORE INSTRUCTIONS ON THE WAY...
pub const INSTRUCTIONS: [Instruction; 49] = [
    Implied     (    nop),
    Op16bit     (  inc16,   Mode16::Register(Reg16::BC)),
    Op16bit     (  addhl,   Mode16::Register(Reg16::BC)),
    Op16bit     (  dec16,   Mode16::Register(Reg16::BC)),
    Op8bit      (   stop,              Mode8::Immediate),
    Op16bit     (  inc16,   Mode16::Register(Reg16::DE)),
    Op8bit      (     jr,              Mode8::Immediate),
    Op16bit     (  addhl,   Mode16::Register(Reg16::DE)),
    Op16bit     (  dec16,   Mode16::Register(Reg16::DE)),
    Op8bit      (   jrnz,              Mode8::Immediate),
    Op16bit     (  inc16,   Mode16::Register(Reg16::HL)),
    Op8bit      (    jrz,              Mode8::Immediate),
    Op16bit     (  addhl,   Mode16::Register(Reg16::HL)),
    Op16bit     (  dec16,   Mode16::Register(Reg16::HL)),
    Op8bit      (   jrnc,              Mode8::Immediate),
    Op16bit     (  inc16,   Mode16::Register(Reg16::SP)),
    Op8bit      (    jrc,              Mode8::Immediate),
    Op16bit     (  addhl,   Mode16::Register(Reg16::SP)),
    Op16bit     (  dec16,   Mode16::Register(Reg16::SP)),
    Implied     (   halt),
    Implied     (  retnz),
    Op16bit     (   jpnz,             Mode16::Immediate),
    Op16bit     (     jp,             Mode16::Immediate),
    Op16bit     ( callnz,             Mode16::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0000)),
    Implied     (   retz),
    Implied     (    ret),
    Op16bit     (    jpz,             Mode16::Immediate),
    Prefix,
    Op16bit     (  callz,             Mode16::Immediate),
    Op16bit     (   call,             Mode16::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0008)),
    Implied     (  retnc),
    Op16bit     (   jpnc,             Mode16::Immediate),
    Op16bit     ( callnc,             Mode16::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0010)),
    Implied     (   retc),
    Implied     (   reti),
    Op16bit     (    jpc,             Mode16::Immediate),
    Op16bit     (  callc,             Mode16::Immediate),
    Op16bit     (    rst,         Mode16::Fixed(0x0018)),
    Op16bit     (    rst,         Mode16::Fixed(0x0020)),
    Op8bit      (  addsp,              Mode8::Immediate),
    Op16bit     (     jp,   Mode16::Register(Reg16::HL)),
    Op16bit     (    rst,         Mode16::Fixed(0x0028)),
    Implied     (     di),
    Op16bit     (    rst,         Mode16::Fixed(0x0030)),
    Implied     (     ei),
    Op16bit     (    rst,         Mode16::Fixed(0x0038)),
];