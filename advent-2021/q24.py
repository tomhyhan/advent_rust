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

    # for i in range(1,10):
    #     reg = {'w': 0, 'x': 0, 'y': 0, 'z': 8599750}
    #     check_bit(i, alu[5],reg)
    #     print(reg["z"] % 26)

# 5 7 8 10 11 12 13
    reg = {'w': 0, 'x': 0, 'y': 0, 'z': 0 }
    # check_bit(7, alu[0], reg)
    # [13, 16, 2, 8,    11, 12, 15]
    # [13]
    # [-9]
    # print(alu[13])
    # [13, 16, 2, 8, 11, 12]
    # [-11, ]

    check_bit(5, alu[0], reg)
    check_bit(3, alu[1], reg)
    check_bit(9, alu[2], reg)
    check_bit(9, alu[3], reg)
    check_bit(9, alu[4], reg)
    check_bit(9, alu[5], reg)
    check_bit(9, alu[6], reg)
    check_bit(5, alu[7], reg)
    check_bit(8, alu[8], reg)
    check_bit(2, alu[9], reg)
    check_bit(9, alu[10], reg)
    check_bit(3, alu[11], reg)
    check_bit(9, alu[12], reg)
    check_bit(9, alu[13], reg)
    x = "53999995829399"
    check_model(alu, "53999995829399")

def test(instructions):
    for i in range(0, 23):
        for j in range(1,10):
            reg = {'w': 0, 'x': 0, 'y': 0, 'z': i}
            check_bit(j, instructions, reg)
                
def check_bit(bit, instructions, reg):
    for instruction in instructions:
        err = run_code(instruction, bit, reg)
        if err:
            print("program err")
            break
    print(reg)

def check_model(alu, model):
    reg = {'w': 0, 'x': 0, 'y': 0, 'z': 0}
    for instructions, inpnum in zip(alu, model):
        for instruction in instructions:
            err = run_code(instruction, inpnum, reg)
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