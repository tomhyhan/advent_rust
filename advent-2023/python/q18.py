from collections import namedtuple

MAX = float("inf")
MIN = float("-inf")

# 0 means R, 1 means D, 2 means L, and 3 means U
DIRECTIONS = {
    'R': (0, 1),
    'L': (0, -1),
    'U': (-1, 0),
    'D': (1, 0),
    '0': (0, 1),
    '2': (0, -1),
    '3': (-1, 0),
    '1': (1, 0),
}


def print_ground(min_x, min_y, max_x, max_y, ground):
    for row in range(min_y, max_y+1):
        # for row in range(min_y, -100):
        s = ""
        for col in range(min_x, max_x+1):
            s += '#' if (row, col) in ground else '.'
        print(s)


def get_outer_area(ground, coord, min_x, min_y, max_x, max_y):
    stack = [coord]
    visited = set()
    area = 0
    while stack:
        row, col = stack.pop()
        if (row, col) in visited:
            continue
        visited.add((row, col))
        area += 1
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nrow, ncol = row + dir[0], col + dir[1]
            if min_y <= nrow <= max_y and min_x <= ncol <= max_x and (nrow, ncol) not in ground:
                stack.append((nrow, ncol))
    # print_ground(min_x, min_y, max_x, max_y, visited)
    return area


def part1(digplans):
    ground = {}
    row, col = 0, 0
    min_x, min_y, max_x, max_y = MAX, MAX, MIN, MIN
    for digplan in digplans:
        head_to = DIRECTIONS[digplan.dir]
        for _ in range(digplan.steps):
            row += head_to[0]
            col += head_to[1]
            ground[(row, col)] = digplan.rgb
            min_x = min(min_x, col)
            max_x = max(max_x, col)
            min_y = min(min_y, row)
            max_y = max(max_y, row)

    min_x -= 1
    max_x += 1
    min_y -= 1
    max_y += 1
    total = abs(max_x - min_x + 1) * abs(max_y - min_y + 1)
    coord = (min_y, min_x)
    outer = get_outer_area(ground, coord, min_x, min_y, max_x, max_y)
    print(total - outer)


def part2(digplans):

    area = 0
    coords = []
    row, col = 0, 0
    length = 0
    for digplan in digplans:
        head_to = DIRECTIONS[digplan.rgb[-1]]
        distance = int(digplan.rgb[1:-1], 16)
        row += head_to[0] * distance
        col += head_to[1] * distance
        length += distance
        coords.append((row, col))
    coords = coords[::-1]
    area = 0
    # coords = [(0, 0), (0, 10), (10, 10), (10, 0)]
    for idx in range(len(coords)):
        first, second = idx % len(coords), (idx+1) % len(coords)
        y, x = coords[first]
        y1, x2 = coords[second]
        area += y * x2 - x * y1
    print(int(area/2), length)
    print(int(int(area/2) + length / 2 + 1))
    #  i + b /2 -1
    # for digplan in digplans:
    #     distance = int(digplan.rgb[1:-1], 16)
    #     dir = digplan.rgb[-1]
    #     print(distance, dir)
    #     pass


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
    # part1(digplans)
    part2(digplans)


solution()
