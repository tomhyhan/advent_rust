from collections import defaultdict

def solution(filename):
    input = open(filename).read()
    alu = []
    instructions = []
    for line in input.split('\n'):
        if line == "add z y":
            instructions.append(line)
            alu.append(instructions)
            instructions = []
        else:
            instructions.append(line)
    # check_model(alu, "99999999999999")
    reg = {'w': 0, 'x': 0, 'y': 0, 'z': 0}
    check_bit(9, alu[0], reg)
    check_bit(8, alu[1], reg)
    check_bit(8, alu[2], reg)
    check_bit(1, alu[3], reg)
    check_bit(8, alu[4], reg)
    check_bit(8, alu[5], reg)
    # check_bit(9, alu[2])
    # check_bit(9, alu[3])
    # check_bit(9, alu[4])

def check_model(alu, model):
    reg = {'w': 0, 'x': 0, 'y': 0, 'z': 0}
    for instructions, inpnum in zip(alu, model):
        for instruction in instructions:
            err = run_code(instruction, inpnum, reg)
            if err:
                print("program err")
                break
        break

def check_bit(bit, instructions, reg):
    reg = {'w': 0, 'x': 0, 'y': 0, 'z': 0}
    for instruction in instructions:
        err = run_code(instruction, bit, reg)
        if err:
            print("program err")
            break
    print(reg)

def run_code(instruction, inpnum, reg):
    instruction = instruction.split(" ")
    opcode = instruction[0]
    match opcode:
        case "inp":
            reg['w'] = int(inpnum)
        case "add":
            reg[instruction[1]] += parse_reg(instruction[2], reg) 
        case "mul":
            reg[instruction[1]] *= parse_reg(instruction[2], reg)
        case "div":
            denominator = parse_reg(instruction[2], reg)
            if denominator == 0:
                return True
            reg[instruction[1]] = int(reg[instruction[1]] / denominator)
        case "mod":
            numerator = reg[instruction[1]]
            denominator = parse_reg(instruction[2], reg)
            if numerator < 0 or denominator <= 0:
                return True
            reg[instruction[1]] = numerator % denominator
        case "eql":
            reg[instruction[1]] = 1 if reg[instruction[1]] == parse_reg(instruction[2], reg) else 0
        case _:
            print("wrong opcode")
            exit(1)
    return False
    
def parse_reg(src,reg):
    return reg[src] if src.isalpha() else int(src)

solution("./inputs/q24.txt")

# inp w
# mul x 0
# add x z
# mod x 26
# eql x w
# eql x 0
# add y 25
# mul z y
# add y w
# add z y

# div z 1
# add x 15
# mul y x
# add y 1
# mul y 0
# add y 13
# mul y x