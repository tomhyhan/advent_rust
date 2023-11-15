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

    for i in range(1,10):
        reg = {'w': 0, 'x': 0, 'y': 0, 'z': 0 }
        check_bit(i, alu[0],reg)
    test(alu[1])
    # du = defaultdict(int)
    # for ins in alu:
    #     for inss in ins:
    #         du[inss] += 1
    
    # for k, v in du.items():
    #     if v < 14:
    #         print(k, v)
    # print(du)
    
# {'w': 9, 'x': 0, 'y': 0, 'z': 18}
def test(instructions):
    for i in range(0, 23):
        for j in range(1,10):
            reg = {'w': 0, 'x': 0, 'y': 0, 'z': i}
            check_bit(j, instructions, reg)
                
def check_bit(bit, instructions, reg):
    # reg = {'w': 0, 'x': 0, 'y': 0, 'z': 0}
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
# add y 13
# mul y x

#  z 1 x 15 y 13
def common(reg, div_z, add_x, add_y):
    reg["x"] += reg["z"]
    reg["x"] % 26
    reg["z"] /= div_z
    reg["x"] += add_x
    reg["x"] = 1 if reg["w"] == reg["x"] else 0
    
    
    pass