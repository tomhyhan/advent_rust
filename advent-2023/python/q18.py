from collections import namedtuple

MAX = float("inf")
MIN = float("-inf")

DIRECTIONS = {
    'R': (0,1),
    'L': (0,-1),
    'U': (-1,0),
    'D': (1,0)
}

def print_ground(min_x, min_y, max_x, max_y, ground):
    for row in range(min_y, max_y+1):
    # for row in range(min_y, -100):
        s = ""
        for col in range(min_x, max_x+1):
            s += '#' if (row,col) in ground else '.'
        print(s)

def within(row,col, ground):
    is_within = False
    i=0
    for i in range(col):
        right = (row-1,i) in ground and (row,i) in ground and (row+1,i) in ground
        top_right = (row,i) in ground and (row,i-1) in ground and (row+1,i) in ground        
        top_left = (row,i) in ground and (row,i+1) in ground and (row+1,i) in ground        
        bot_right =  (row,i) in ground and (row,i-1) in ground and (row-1,i) in ground
        bot_left =  (row,i) in ground and (row,i+1) in ground and (row-1,i) in ground
        if right or top_right or bot_right or top_left or bot_left:
            is_within = not is_within
    return is_within

def calc_inner_spaces(min_x, min_y, max_x, max_y, ground):
    area = 0
    inner = {}
    for row in range(min_y, max_y+1):
        for col in range(min_x, max_x+1):
            if (row,col) in ground:
                continue
            # print(row,col)
            if within(row,col,ground):
                area += 1                
                inner[(row,col)] = '#'
    print(ground[(13,40)])
    return inner

def get_outer_area(ground, coord, min_x, min_y, max_x, max_y):
    stack = [coord]
    visited = set()
    area = 0
    while stack:
        row, col  = stack.pop()
        print(row,col)
        if (row,col) in visited:
            continue
        visited.add((row,col))
        area += 1
        for dir in [(0,1),(0,-1),(1,0),(-1,0)]:
            nrow, ncol = row + dir[0], col + dir[1]
            print(nrow, ncol)
            if min_y <= nrow <= max_y and min_x <= ncol <= max_x and (nrow,ncol) not in ground:
                stack.append((nrow,ncol))
    # print_ground(min_x, min_y, max_x, max_y, visited)
    return area

def part1(digplans):
    ground = {}
    row, col = 0,0
    min_x, min_y, max_x, max_y = MAX, MAX, MIN, MIN 
    for digplan in digplans:
        head_to = DIRECTIONS[digplan.dir]
        for _ in range(digplan.steps):
            row += head_to[0]
            col += head_to[1]
            ground[(row,col)] = digplan.rgb
            min_x = min(min_x, col)   
            max_x = max(max_x, col)   
            min_y = min(min_y, row)   
            max_y = max(max_y, row)   

    inner = calc_inner_spaces(min_x, min_y, max_x, max_y, ground)
    ground.update(inner)
    # print_ground(min_x, min_y, max_x, max_y, ground)
    print(len(ground))
    # 22717 X
    # 22900 X
    # 42410
    pass    

# 1j
# (-0-1j)
# (1+0j)
# (-1+0j)

def parse(lines):
    Digplan = namedtuple("Digplan", ["dir", "steps", "rgb"])
    digplans = []
    for line in lines:
        dir, steps, rgb = line.split()
        digplans.append(Digplan(dir, int(steps), rgb.strip('(').strip(')')))
    return digplans

def solution():
    filename = "./inputs/q18.txt"
    lines = open(filename).read().split('\n')
    digplans = parse(lines)
    part1(digplans)

solution()