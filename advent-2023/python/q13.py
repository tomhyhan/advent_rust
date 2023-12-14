def parse(filename):
    patterns = open(filename).read().split('\n\n')
    ps = []
    for pattern in patterns:
        lines = pattern.split('\n')
        rows = [line for line in lines]
        cols = [''.join(map(lambda line: line[col], lines)) for col in range(len(lines[0]))]
        ps.append([rows, cols])
    return ps

def is_match(pattern, s):
    left, right = s - 1, s 
    while left >= 0 and right < len(pattern):
        if pattern[left] != pattern[right]:
            return False
        left -= 1
        right += 1
    return True

def does_reflect(pattern, part2, original = 0):
    for i in range(1, len(pattern)):
        if is_match(pattern, i):
            if part2 and i == original:
                continue
            return i
    return False

def part1(patterns):
    total = 0
    mirror_pos = {}
    for idx, pattern in enumerate(patterns):
        rows, cols = pattern
        mirror = does_reflect(rows, False)
        if mirror:
            total += mirror * 100
            mirror_pos[idx] = mirror * 100
        else:
            mirror = does_reflect(cols, False)
            total += mirror
            mirror_pos[idx] = mirror 
    print(total)    
    return mirror_pos

def check_smudge(current, mirror_score, is_col):
    for row in range(len(current)):
        for col in range(len(current[row])):
            temp = current[row][col] 
            current[row][col] = '.' if current[row][col] == '#' else '#' 
            original = mirror_score if is_col else mirror_score // 100
            if mirror:= does_reflect(current, True, original):
                mirror = mirror if is_col else mirror * 100
                return mirror
                    
            current[row][col] = temp
    return False
    
def part2(mirror_pos, patterns):
    total = 0
    for idx, pattern in enumerate(patterns):
        mirror_score = mirror_pos[idx]
        for is_col, current in enumerate(pattern):
            mirror = check_smudge([list(line) for line in current], mirror_score, is_col)
            if mirror:
                total += mirror
                break
    print(total)

def solution():
    # improve with xor, and diff == 1
    # new_trick: list(zip(*test.split('\n')))
    filename = "./inputs/q13.txt"
    patterns = parse(filename)
    mirror_pos = part1(patterns)
    part2(mirror_pos, patterns)

solution()