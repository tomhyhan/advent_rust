from collections import deque
from itertools import product

def mahattan(p1, p2):
    print(abs(p1[0] - p2[0]) + abs(p1[1] - p2[1]))

# 3 
#  
def solution():
    input = open("./inputs/q19.txt").read()
    t = parse_scanners(input)
    scanners = deque(parse_scanners(input))

    x = scanners.popleft()
    y = scanners.popleft()
    print(list(product(x, y)))

def parse_scanners(input):
    return [set(tuple(map(int, beacon.split(',')))
                for beacon in scanner.split('\n') if beacon[1] != '-')
            for scanner in input.split('\n\n')]

solution()

# mahattan((3,3), (4,1))
# mahattan((3,3), (0,2))
# mahattan((-2,1), (-1,-1))
# mahattan((-2,1), (-5,0))