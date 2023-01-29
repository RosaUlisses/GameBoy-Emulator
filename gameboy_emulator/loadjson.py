import sys
import json

def control(name, op1, op2):
  opcode = name
  if op1 in ("Z", "NZ", "C", "NC"):
    opcode += op1
    op1 = op2
  match op1:
    case "r8":
      mode = "Op8bit"
      ops = [(8, "Immediate")]
    case "a16":
      mode = "Op16bit"
      ops = [(16, "Immediate")]
    case "(HL)":
      mode = "Op16bit"
      ops = [(16, f"Register(Reg16::HL)")]
    case None:
      mode = "Implied"
      ops = []
    case num:
      num = num[:2]
      mode = "Op16bit"
      ops = [(16, f"Fixed(0x00{num})")]
  return mode, opcode, ops 
  
def misc(name, op1, op2):
  opcode = name
  match op1:
    case "0":
      mode = "Op8bit"
      ops = [(8, "Immediate")]
    case "CB":
      mode = "Prefix"
      opcode = ""
      ops = []
    case None:
      mode = "Implied"
      ops = []
  return mode, opcode, ops

# Implied (FnImplied),
# Op8bit (FnOp8bit, AddressingMode8bit),
# Op8bit8bit (FnOp8bit8bit, AddressingMode8bit, AddressingMode8bit),
# Op16bit (FnOp16bit, AddressingMode16bit),
# Op16bit16bit (FnOp16bit16bit, AddressingMode16bit, AddressingMode16bit),
# PrefixExtended,

# pub enum AddressingMode8bit {
#     Register(Registers8bit),
#     Immediate,
#     Address,
#     Fixed(u8),
# }
# pub enum AddressingMode16bit {
#     Register(Registers16bit),
#     Immediate,
#     Address,
#     Fixed(u16),
# }

def alu16(name, op1, op2):
  # print(f"{name} {op1},{op2}")
  opcode = name
  if op2 is None:
    opcode += "16"
    mode = "Op16bit"
    ops = [(16, f"Register(Reg16::{op1})")]
  elif op1 == "HL":
    opcode += "hl"
    mode = "Op16bit"
    ops = [(16, f"Register(Reg16::{op2})")]
  elif op1 == "SP":
    opcode += "sp"
    mode = "Op8bit"
    ops = [(8, f"Immediate")]
  return mode, opcode, ops

def lsm16(name, op1, op2):
  return "Implied", "NOP", []
  print(f"{name} {op1},{op2}")

def alu8(name, op1, op2):
  return "Implied", "NOP", []
  print(f"{name} {op1},{op2}")

def lsm8(name, op1, op2):
  return "Implied", "NOP", []
  print(f"{name} {op1},{op2}")

def rsb8(name, op1, op2):
  return "Implied", "NOP", []
  print(f"{name} {op1},{op2}")

def generate_operand(size, optype):
  return f"Mode{size}::{optype}"

def generate_instr(name, group, op1, op2):
  match group:
    case "control/br":
      mode, opcode, ops = control(name, op1, op2)
    case "control/misc":
      mode, opcode, ops = misc(name, op1, op2)
    case "x16/alu":
      mode, opcode, ops = alu16(name, op1, op2)
    case "x16/lsm":
      mode, opcode, ops = lsm16(name, op1, op2)
      return
    case "x8/alu":
      mode, opcode, ops = alu8(name, op1, op2)
      return
    case "x8/lsm":
      mode, opcode, ops = lsm8(name, op1, op2)
      return
    case "x8/rsb":
      mode, opcode, ops = rsb8(name, op1, op2)
      return
    case _: raise ValueError()

  params = ','.join(
    [f"{opcode.lower():>7}"] + 
    [f"{generate_operand(s, t):>30}" for s, t in ops]
  )
  print(f"    {mode:12}({params}),")

def main(argc, argv):
  with open("src/opcodes.json", "r") as file:
    instructions = json.load(file)
  
  for instr in instructions["unprefixed"].values():
    name  = instr["mnemonic"]
    group = instr["group"]
    op1   = instr["operand1"] if "operand1" in instr else None
    op2   = instr["operand2"] if "operand2" in instr else None
    generate_instr(name, group, op1, op2)

  return 0

sys.exit(main(len(sys.argv), sys.argv))

# control/br:
#   operand1:
#     (HL)
#     00H
#     08H
#     10H
#     18H
#     20H
#     28H
#     30H
#     38H
#     C
#     NC
#     NZ
#     Z
#     a16
#     r8
#   operand2:
#     a16
#     r8
# control/misc:
#   operand1:
#     0
#     CB
#   operand2:
# x16/alu:
#   operand1:
#     BC
#     DE
#     HL
#     SP
#   operand2:
#     BC
#     DE
#     HL
#     SP
#     r8
# x16/lsm:
#   operand1:
#     (a16)
#     AF
#     BC
#     DE
#     HL
#     SP
#   operand2:
#     HL
#     SP
#     SP+r8
#     d16
# x8/alu:
#   operand1:
#     (HL)
#     A
#     B
#     C
#     D
#     E
#     H
#     L
#     d8
#   operand2:
#     (HL)
#     A
#     B
#     C
#     D
#     E
#     H
#     L
#     d8
# x8/lsm:
#   operand1:
#     (BC)
#     (C)
#     (DE)
#     (HL)
#     (HL+)
#     (HL-)
#     (a16)
#     (a8)
#     A
#     B
#     C
#     D
#     E
#     H
#     L
#   operand2:
#     (BC)
#     (C)
#     (DE)
#     (HL)
#     (HL+)
#     (HL-)
#     (a16)
#     (a8)
#     A
#     B
#     C
#     D
#     E
#     H
#     L
#     d8
# x8/rsb:
#   operand1:
#     (HL)
#     0
#     1
#     2
#     3
#     4
#     5
#     6
#     7
#     A
#     B
#     C
#     D
#     E
#     H
#     L
#   operand2:
#     (HL)
#     A
#     B
#     C
#     D
#     E
#     H
#     L