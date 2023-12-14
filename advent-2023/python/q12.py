from itertools import groupby
from collections import deque

def find_questions(line):
    return deque([idx for idx, char in enumerate(line) if char == '?'])

def gen_patterns(q_len):
    patterns = []
    for i in range(1 << q_len):
        patterns.append(bin(i)[2:].zfill(q_len))
    return patterns

def create_possible_patterns(patterns, spring, question_idxs):
    possibles = []
    for pattern in patterns:
        new_pattern = list(spring)
        for state, loc in zip(pattern, question_idxs):
            new_pattern[loc] = '.' if state == '0' else '#'
        possibles.append(new_pattern)
    return possibles

def find_matches1(possibles, arrange):
    matches = 0
    for possible in possibles:
        if list(map(len, [state for state in ''.join(possible).split('.') if state])) == arrange:
            matches += 1
    return matches
    
def part1(lines):
    total = 0
    for line in lines:
        spring, arrange = line.split()
        spring_end = spring[-1]
        new_spring = spring
        # if spring_end == '.':
        #     new_spring = '?' + new_spring
        # elif spring_end == '?':
            
        question_idxs = find_questions(spring)
        patterns = gen_patterns(len(question_idxs))

        possibles = create_possible_patterns(patterns, spring,question_idxs)

        arrange = list(map(int, arrange.split(',')))
        matches = find_matches1(possibles, arrange)
        total += matches            
    print(total)

def group_spring(spring):
    grp = []
    pair = 0
    for char in spring:
        if char == "?":
            break
        
        if char == '#':
            pair += 1
        else:
            if pair > 0:
                grp.append(pair)
            pair = 0
    return grp    
    
def can_continue(spring, arrange):
    grp = group_spring(spring)
    # print("spring", spring)
    # print("grp", grp)
    if not grp:
        return True
    for a,b in zip(grp, arrange):
        if a > b:
            return False
    return True

def find_matches(spring, arrange, question_idxs):
    if not can_continue(spring, arrange):
        return 0
    
    if not question_idxs:
        if group_spring(spring) == arrange:
            return 1
        else:
            return 0

    matches = 0
    for state in ['.','#']:
        poped = question_idxs.popleft()
        spring[poped] = state
        matches += find_matches(spring, arrange, question_idxs)
        spring[poped] = '?'
        question_idxs.appendleft(poped)
    return matches

def create_matches(spring, arrange, start, memo):
    key = (spring, tuple(arrange), start)
    if key in memo:
        return memo[key]

    if not arrange:
        if '#' not in spring:
            return 1
        else:
            return 0
    
    matches = 0
    for i in range(start, len(spring)):
        poped = arrange.popleft()
        new_spring = list(spring)
        matching_p = ['@'] * poped
        for j in range(i,min(len(spring),i+poped)):
            if new_spring[j] in "#?":
                new_spring[j] = '@'
        current_p = new_spring[i:i+poped]
        left_is_dot = True if i == 0 else new_spring[i-1] in '.?'
        right_is_dot = True if i+poped >= len(new_spring) -1 else new_spring[i+poped] in '.?'
        is_match = matching_p == current_p
        if left_is_dot and right_is_dot and is_match:
            matches += create_matches(''.join(new_spring), arrange, i+poped+1, memo)
        arrange.appendleft(poped)
    memo[key] = matches
    return matches

def go_deep(spring, arrange, si, ai, memo):
    if (si,ai) in memo:
        return memo[(si,ai)]
    total = 0

    if si >= len(spring):
        return ai == len(arrange)

    if spring[si] in '.?':
        total += go_deep(spring, arrange, si + 1, ai, memo)
    
    if spring[si] in '#?' and ai < len(arrange):
        p = int(arrange[ai])
        is_match = si + p <= len(spring) and '.' not in spring[si:si+p]
        is_right_dot = True if (si + p) >= len(spring) else spring[si + p] in '.?'
        if is_match and is_right_dot:       
            total += go_deep(spring, arrange, si + p + 1, ai + 1, memo)
            
    memo[(si,ai)] = total
    return total
    
def part2(lines):
    total = 0
    factor = 5
    for line in lines:
        spring, arrange = line.split()
        spring = '?'.join([spring] * factor)
        arrange = ','.join([arrange] * factor)
        memo = {}
        matches = go_deep(spring, arrange.split(','),0, 0, memo)
        total += matches
    print(total)
def solution():
    filename = "./inputs/q12.txt"
    lines = open(filename).read().split('\n')
    # part1(lines)
    part2(lines)

solution()
""" failed 1st attempt
def part1(lines):
    total = 0
    for line in lines:
        spring, arrange = line.split()
        question_idxs = find_questions(spring)
        patterns = gen_patterns(len(question_idxs))
        possibles = []
        for pattern in patterns:
            new_pattern = list(spring)
            for state, loc in zip(pattern, question_idxs):
                new_pattern[loc] = '.' if state == '0' else '#'
            possibles.append(new_pattern)
        matches = 0
        arrange = list(map(int, arrange.split(',')))
        for possible in possibles:
            if list(map(len, [state for state in ''.join(possible).split('.') if state])) == arrange:
                matches += 1
        total += matches            
    print(total)
"""

""" failed 2nd attempt
def part2(lines):
    for line in lines:
        spring, arrange = line.split()
        question_idxs = find_questions(spring)
        arrange = list(map(int, arrange.split(',')))
        matches = find_matches(list(spring), arrange, question_idxs)
        print(matches)
"""

""" failed 3rd attempt :):)
def part2(lines):
    total = 0
    factor = 5
    for line in lines:
        spring, arrange = line.split()
        spring = '?'.join([spring] * factor)
        arrange = ','.join([arrange] * factor)
        arrange = deque([int(a) for a in arrange.split(',')])
        memo = {}
        matches = create_matches(spring, arrange, 0, memo)
        total += matches
    print(total)
"""