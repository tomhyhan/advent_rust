def parse(filename):
    patterns = open(filename).read().split('\n\n')
    ps = []
    for pattern in patterns:
        lines = pattern.split('\n')
        rows = [line for line in lines]
        cols = [''.join(map(lambda line: line[col], lines)) for col in range(len(lines[0]))]
        ps.append([rows, cols])
    return ps
    pass

def is_match(pattern, s):
    left, right = s - 1, s 
    while left >= 0 and right < len(pattern):
        if pattern[left] != pattern[right]:
            return False
        left -= 1
        right += 1
    return True

def does_reflect(pattern):
    for i in range(1, len(pattern)):
        if is_match(pattern, i):
            return i
    return False

def part1(patterns):
    total = 0
    for pattern in patterns:
        rows, cols = pattern
        mirror = does_reflect(rows)
        if mirror:
            total += mirror * 100
        else:
            total += does_reflect(cols)
    print(total)    

def solution():
    filename = "./inputs/q13.txt"
    patterns = parse(filename)
    part1(patterns)
    
solution()