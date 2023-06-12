import re

def do_cmd(fn):
    def final(before, instr):
        after = list(before)
        after[instr[3]] = fn(before, instr[1], instr[2])
        return after
    return final

addr = do_cmd(lambda before,x,y: before[x]+before[y])
addi = do_cmd(lambda before,x,y: before[x]+y)
mulr = do_cmd(lambda before,x,y: before[x]*before[y])
muli = do_cmd(lambda before,x,y: before[x]*y)
banr = do_cmd(lambda before,x,y: before[x] & before[y])
bani = do_cmd(lambda before,x,y: before[x] & y)
borr = do_cmd(lambda before,x,y: before[x] | before[y])
bori = do_cmd(lambda before,x,y: before[x] | y)
setr = do_cmd(lambda before,x,y: before[x])
seti = do_cmd(lambda before,x,y: x)
gtir = do_cmd(lambda before,x,y: 1 if x > before[y] else 0)
gtri = do_cmd(lambda before,x,y: 1 if before[x] > y else 0)
gtrr = do_cmd(lambda before,x,y: 1 if before[x] > before[y] else 0)
eqir = do_cmd(lambda before,x,y: 1 if x == before[y] else 0)
eqri = do_cmd(lambda before,x,y: 1 if before[x] == y else 0)
eqrr = do_cmd(lambda before,x,y: 1 if before[x] == before[y] else 0)

cmds = [ addr, addi
       , mulr, muli
       , banr, bani
       , borr, bori
       , setr, seti
       , gtir, gtri, gtrr
       , eqir, eqri, eqrr
       ]

options = {}
for code in range(16):
    options[code] = list(enumerate(cmds))

lines,program = open('q16.txt').read().strip().split('\n\n\n')
lines = lines.strip().split('\n')
ans = 0
for i in range(0, len(lines), 4):
    if 'Before' in lines[i]:
        assert 'After:' in lines[i+2]
        before = list(map(int, re.findall('-?\d+', lines[i])))
        instr = list(map(int, re.findall('-?\d+', lines[i+1])))
        after = list(map(int, re.findall('-?\d+', lines[i+2])))
        options[instr[0]] = [(idx,fn) for (idx,fn) in options[instr[0]] if fn(before,instr)==after]

        matches = 0
        for idx,cmd in options[instr[0]]:
            if cmd(before, instr) == after:
                matches += 1
        if matches >= 3:
            ans += 1

for _ in range(16):
    for code in range(16):
        if len(options[code]) == 1:
            for other_code in range(16):
                if other_code != code:
                    options[other_code] = [(idx,fn) for (idx,fn) in options[other_code] if idx!=options[code][0][0]]
        
print(options)

#for code in range(16):
#    print code, options[code]

registers = [0,0,0,0]
for line in program.strip().split('\n'):
    instr = list(map(int, re.findall('-?\d+', line)))
    old_registers = list(registers)
    registers = options[instr[0]][0][1](registers, instr)
# print registers[0]