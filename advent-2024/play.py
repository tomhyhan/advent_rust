# 246810588779586
import heapq
from collections import defaultdict

codes = open("input/day21.txt").read().splitlines()

print(codes)

NUMERIC_PAD = {
    (0, 0): '7', (0, 1): '8', (0, 2): '9',
    (1, 0): '4', (1, 1): '5', (1, 2): '6',
    (2, 0): '1', (2, 1): '2', (2, 2): '3',
    (3, 1): '0', (3, 2): 'A',
}

DIR_PAD = {
    (0, 1): '^', (0, 2): 'A',
    (1, 0): '<', (1, 1): 'v', (1, 2): '>'
}

DIRECTIONS = [(-1, 0, '^'), (0, 1, '>'), (1, 0, 'v'), (0, -1, '<')]


def create_shortest_paths(pad):
    paths = defaultdict(list)
    for pt, ch in pad.items():
        queue = [(0, "", *pt)]
        seen = defaultdict(lambda: float('inf'))
        while queue:
            distance, path, row, col = heapq.heappop(queue)

            key = (row, col)
            if distance > seen[key]:
                continue
            seen[key] = distance
            paths[(ch, pad[key])].append(path + 'A')

            for dir in DIRECTIONS:
                nrow = row + dir[0]
                ncol = col + dir[1]
                p = dir[2]
                if (nrow, ncol) in pad:
                    heapq.heappush(queue, (distance+1, path+p, nrow, ncol))
    return paths


num_paths = create_shortest_paths(NUMERIC_PAD)
dir_paths = create_shortest_paths(DIR_PAD)

for code in codes:

    num_paths
    break

for k, v in dir_paths.items():
    print(k, v)
    print()
