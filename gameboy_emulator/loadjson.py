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


def alu16(name, op1, op2):
  opcode = name
  if op2 is None:
    opcode += "16"
    mode = "Op16bit"
    ops = [(16, register16(op1))]
  elif op1 == "HL":
    opcode += "hl"
    mode = "Op16bit"
    ops = [(16, register16(op2))]
  elif op1 == "SP":
    opcode += "sp"
    mode = "Op8bit"
    ops = [(8, f"Immediate")]
  return mode, opcode, ops


def lsm16(name, op1, op2):
  mode, opcode, ops = "Implied", "NOP", []
  opcode = name

  match op2:
    case None:
      mode = "Op16bit"
      ops = [(16, register16(op1))]
    case "d16":
      mode = "Op16bit16bit"
      opcode += "16"
      ops = [
        (16, register16(op1)),
        (16, f"Immediate"),
      ]
    case "HL":
      mode = "Op16bit16bit"
      opcode += "16"
      ops = [
        (16, register16(op1)),
        (16, register16(op2)),
      ]
    case "SP+r8":
      mode = "Op8bit"
      opcode += "HL"
      ops = [(8, "Immediate")]
    case "SP":
      mode = "Op16bit16bit"
      opcode += "16"
      ops = [
        (16, f"Address"),
        (16, register16(op2)),
      ]
  return mode, opcode, ops


def alu8(name, op1, op2):
  mode = "Op8bit"
  opcode = name

  if(op1 == "A" and op2 is not None):
    op1 = op2

  match op1:
    case "(HL)":
      ops = [(8, "Indirect(Reg16::HL)")]
    case None:
      mode = "Implied"
      ops = []
    case "d8":
      ops = [(8, "Immediate")]
    case reg:
      ops = [(8, f"Register(Reg8::{reg})")]

  return mode, opcode, ops


def lsm8(name, op1, op2):
  opcode = "LD"
  mode = "Op8bit8bit"
  ops = []

  for operand in (op1, op2):
    match operand:
      case "(HL+)":
        op = "Indirect(Reg16::HL)"
        opcode += "I"
      case "(HL-)":
        op = "Indirect(Reg16::HL)"
        opcode += "D"
      case "d8":
        op = "Immediate"
      case "(a8)":
        op = "IndexedImm"
      case "(a16)":
        op = "Address"
      case "(C)":
        op = "IndexedC"
      case reg:
        if reg[0] == "(":
          op = f"Indirect(Reg16::{reg[1:3]})"
        else:
          op = f"Register(Reg8::{reg})"
    
    ops.append((8, op))
  
  return mode, opcode, ops

def rsb8(name, op1, op2):
  opcode = name[:-1]
  mode = "Op8bit"
  ops = [(8, "Register(Reg8::A)")]

  return mode, opcode, ops


def generate_operand(size, optype):
  return f"Mode{size}::{optype}"


def register16(reg):
  if reg == "SP":
    return "StackPointer"
  else:
    return f"Register(Reg16::{reg})"


def generate_instr(name, group, op1, op2):
  match group:
    case "control/br":
      mode, opcode, ops = control(name, op1, op2)
      # return
    case "control/misc":
      mode, opcode, ops = misc(name, op1, op2)
      # return
    case "x16/alu":
      mode, opcode, ops = alu16(name, op1, op2)
      # return
    case "x16/lsm":
      mode, opcode, ops = lsm16(name, op1, op2)
      # return
    case "x8/alu":
      mode, opcode, ops = alu8(name, op1, op2)
      # return
    case "x8/lsm":
      mode, opcode, ops = lsm8(name, op1, op2)
      # return
    case "x8/rsb":
      mode, opcode, ops = rsb8(name, op1, op2)
      # return

  params = ','.join(
    [f"{opcode.lower():>7}"] + 
    [f"{generate_operand(s, t):>30}" for s, t in ops]
  )
  if opcode == "":
    print(f"    {mode:12},")
  else:
    print(f"    {mode:12}({params}),")


def generate_prefixed(name, op1, op2):
  mode = "Op8bit"
  opcode = name
  ops = []

  if op2 is not None:
    mode = "Op8bit8bit"
    ops.append((8, f"Fixed({op1})"))
    op1 = op2
  
  match op1:
    case "(HL)":
      ops.append((8, "Indirect(Reg16::HL)"))
    case reg:
      ops.append((8, f"Register(Reg8::{reg})"))

  params = ','.join(
    [f"{opcode.lower():>7}"] + 
    [f"{generate_operand(s, t):>30}" for s, t in ops]
  )
  if opcode == "":
    print(f"    {mode:12},")
  else:
    print(f"    {mode:12}({params}),")


def main(argc, argv):
  with open("src/opcodes.json", "r") as file:
    instructions = json.load(file)
  
  for i in range(256):
    opcode = f"0x{i:02x}"
    if opcode not in instructions["unprefixed"]:
      print("    Invalid,")
    else:
      instr = instructions["unprefixed"][opcode]

      name  = instr["mnemonic"]
      group = instr["group"]
      op1   = instr["operand1"] if "operand1" in instr else None
      op2   = instr["operand2"] if "operand2" in instr else None

      generate_instr(name, group, op1, op2)

  for i in range(256):
    opcode = f"0x{i:02x}"
    instr = instructions["cbprefixed"][opcode]

    name  = instr["mnemonic"]
    op1   = instr["operand1"] if "operand1" in instr else None
    op2   = instr["operand2"] if "operand2" in instr else None

    generate_prefixed(name, op1, op2)


  return 0


sys.exit(main(len(sys.argv), sys.argv))


# Implied (FnImplied),
# Op8bit (FnOp8bit, AddressingMode8bit),
# Op8bit8bit (FnOp8bit8bit, AddressingMode8bit, AddressingMode8bit),
# Op16bit (FnOp16bit, AddressingMode16bit),
# Op16bit16bit (FnOp16bit16bit, AddressingMode16bit, AddressingMode16bit),
# PrefixExtended,

# pub enum AddressingMode8bit {
#     Register(Reg8),
#     Immediate,
#     Address,
#     Fixed(u8),
# }
# pub enum AddressingMode16bit {
#     Register(Reg16),
#     Immediate,
#     Address,
#     Fixed(u16),
# }
