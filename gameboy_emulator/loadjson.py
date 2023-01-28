import sys
import json

def main(argc, argv):
  with open("src/opcodes.json", "r") as file:
    instructions = json.load(file)
  
  operandtypes = dict()
  for insttype in instructions.values():
    for opcode in insttype.values():
      key = opcode["group"]
      if key not in operandtypes:
        operandtypes[key] = { "operand1": set(), "operand2": set() }
      
      if "operand1" in opcode:
        operandtypes[key]["operand1"].add(opcode["operand1"])
      if "operand2" in opcode:
        operandtypes[key]["operand2"].add(opcode["operand2"])
  
  for k in sorted(operandtypes.keys()):
    group = operandtypes[k]
    print(f"{k}:")
    for operand, optypes in group.items():
      print(f"\t{operand}:")
      for optype in sorted(optypes):
        print(f"\t\t{optype}")
  return 0

sys.exit(main(len(sys.argv), sys.argv))