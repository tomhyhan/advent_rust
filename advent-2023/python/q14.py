def part1(lines):
    vertical_map = transpos(lines)
    print(vertical_map)
    moved = True
    while moved: 
        moved = False
        for line in vertical_map:
            for i in range(len(line) - 1):
                if line[i] == '.' and line[i+1] == 'O': 
                    line[i], line[i+1] = line[i+1], line[i]
                    moved = True
    print(vertical_map)
    
def transpos(lines):
    return [zip(*lines)]

def solution():
    filename = "./inputs/q14.txt"
    lines = [list(line) for line in open(filename).read().split('\n')]
    part1(lines)


solution()