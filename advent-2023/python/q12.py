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
    print(4  * (5 ** 4))

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

def create_matches(spring, arrage, start):
    
    
    
    total = 0
    current_arr = arrage.popleft()
    current_pattern = ['#'] * current_arr + ['.']
    for i in range(start, len(spring), current_arr):
        pattern = list(spring[i:i+current_arr + 1])
        for j in range(len(pattern)-1):
            if pattern[j] == '?':
                pattern[j] = '#'
        pattern[-1] = '.' if pattern[-1] == '?' else pattern[-1]
        if current_pattern == pattern:
            total += create_matches(spring, pattern, i+2)
    arrage.appendleft(current_arr)
    return total
def part2(lines):
    for line in lines:
        spring, arrange = line.split()
        arrange = deque([int(a) for a in arrange.split(',')])
        create_matches(spring, arrange, 0)
        
def solution():
    filename = "./inputs/q12.txt"
    lines = open(filename).read().split('\n')
    # part1(lines)
    part2(lines)

solution()
"""
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

"""
def part2(lines):
    for line in lines:
        spring, arrange = line.split()
        question_idxs = find_questions(spring)
        arrange = list(map(int, arrange.split(',')))
        matches = find_matches(list(spring), arrange, question_idxs)
        print(matches)
"""