from dataclasses import dataclass
from z3 import *

@dataclass
class Hailstone:
    pos : list
    vel : list
    
    def add_vel(self):
        self.pos[0] += self.vel[0]
        self.pos[1] += self.vel[1]
        self.pos[2] += self.vel[2]

def past(x,y, stone):
    x_inc = stone.vel[0] > 0
    y_inc = stone.vel[1] > 0
    if x_inc and y_inc:
        if stone.pos[0] > x and stone.pos[1] > y:
            return True
    elif x_inc and not y_inc:
        if stone.pos[0] > x and stone.pos[1] < y:
            return True
    elif not x_inc and y_inc:
        if stone.pos[0] < x and stone.pos[1] > y:
            return True
    else:
        if stone.pos[0] < x and stone.pos[1] < y:
            return True
    return False

def find_intersection_2d(stone1, stone2):
    # y=m1x+b1 = m2x + b2
    # x = (b2 - b1) / (m1 - m2)
    slope1 = stone1.vel[1] / stone1.vel[0] 
    slope2 = stone2.vel[1] / stone2.vel[0] 
    if slope1 == slope2: 
        raise ZeroDivisionError 
    b1 = stone1.pos[1] - (slope1 * stone1.pos[0]) 
    b2 = stone2.pos[1] - (slope2 * stone2.pos[0]) 
    x = (b2-b1) / (slope1 - slope2)
    y = slope1 * x + b1
    if past(x,y, stone1) or past(x,y, stone2):
        return -1, -1
    return x, y

def part1(stones):
    total = 0
    # minr, maxr = 7, 27
    minr, maxr = 200000000000000, 400000000000000
    for i in range(len(stones)):
        for j in range(i+1, len(stones)):
            stone1, stone2 = stones[i], stones[j]
            try:
                x,y = find_intersection_2d(stone1, stone2)
                if minr <= x <= maxr and minr <= y <= maxr:
                    total += 1
            except ZeroDivisionError:
                pass
    print(total)
    
def parse(filename):
    return [Hailstone(*map(lambda x: list(map(int, x)), map(lambda x: x.split(','), line.split(" @ ")))) for line in open(filename).read().split('\n')]

def part2(stones):
    real = lambda x: Real(x)
    x = real('x')
    y = real('y')
    z = real('z')
    vx = real('vx')
    vy = real('vy')
    vz = real('vz')

    solver = Solver()
    for i, stone in enumerate(stones):
        t = real(f"t{i}")
        solver.insert(x + vx * t == stone.pos[0] + stone.vel[0] * t)
        solver.insert(y + vy * t == stone.pos[1] + stone.vel[1] * t)
        solver.insert(z + vz * t == stone.pos[2] + stone.vel[2] * t)

    print(solver.check())
    print(solver.model().eval(x+y+z))    

def solution():
    filename = "./inputs/q24.txt"
    stones = parse(filename)
    part1(stones)
    part2(stones)
    
solution()