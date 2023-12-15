import json

def part1(lines):
    vertical_map = transpos(lines)
    platform = tilt(vertical_map, True) 
    horizontal_map = transpos(platform)
    print(get_total_load(horizontal_map))    
    
def tilt(platform, north_west):
    moved = True
    current, next_ = ('.', 'O') if north_west else ('O', '.') 
    while moved:
        moved = False
        for line in platform:
            for i in range(len(line) - 1):
                if line[i] == current and line[i+1] == next_:
                    line[i], line[i+1] = line[i+1], line[i]
                    moved = True
    return platform
    
def get_total_load(platform):
    length = len(platform)
    total_load = 0
    for idx, r in enumerate(platform):
        load = length - idx
        total_load += r.count('O') * load
    return total_load


def transpos(lines):
    return [list(row) for row in zip(*lines)]

def cycle(platform):
    for dir in [True,True,False,False]:
        transformed = transpos(platform) 
        platform = tilt(transformed, dir)
    return platform

def part2(platform):
    platforms = {}
    values = []
    idx = 0
    while True:
        key = json.dumps(platform)
        if key in platforms:
            break
        platforms[key]= idx
        platform = cycle(platform)
        total = get_total_load(platform)
        values.append(total)
        idx += 1
    start, end = platforms[json.dumps(platform)], idx
    offset = (1000000000 - start - 1) % (end - start)
    print(values[start+offset])

def solution():
    filename = "./inputs/q14.txt"
    lines = [list(line) for line in open(filename).read().split('\n')]
    part1(lines)
    part2(lines)
solution()
