# | is a vertical pipe connecting north and south.
# - is a horizontal pipe connecting east and west.
# L is a 90-degree bend connecting north and east.
# J is a 90-degree bend connecting north and west.
# 7 is a 90-degree bend connecting south and west.
# F is a 90-degree bend connecting south and east.

NEIGHBORS = {
    '|': [(1,0),(-1,0)],
    '-': [(0,-1),(0,1)],
    'L': [(-1,0),(0,1)],
    'J': [(-1,0),(0,-1)],
    '7': [(1,0),(0,-1)],
    'F': [(1,0),(0,1)],
}


def is_connected(maze,row,col,dir):
    nrow, ncol = row + dir[0], col + dir[1]
    match dir:
        case (1,0):
            if maze[nrow][ncol] in ['|', 'L', 'J']:
                return True
        case (-1,0):
            if maze[nrow][ncol] in ['|', 'F', '7']:
                return True
        case (0,1):
            if maze[nrow][ncol] in ['-', '7', 'J']:
                return True
        case (0,-1):
            if maze[nrow][ncol] in ['-', 'F', 'L']:
                return True
        case _:
            raise Exception("unknonw direction")
    return False

def connecting_to_start(maze):
    connections = []
    for row in range(len(maze)):
        for col in range(len(maze[0])):
            if maze[row][col] == 'S':
                for dir in [(-1,0),(1,0),(0,1),(0,-1)]:
                    if is_connected(maze, row, col, dir):
                        connections.append((row + dir[0], col + dir[1]))    
                return (row,col), connections

def part1(maze):
    start, s_con = connecting_to_start(maze)
    path = 1
    node = s_con[0]
    visited = set([start])
    while node not in visited:
        row, col = node
        print(node)
        current = maze[node[0]][node[1]]
        visited.add((row,col))
        for neighbor in NEIGHBORS[current]:
            nrow, ncol = neighbor
            key = (row + nrow, col + ncol)
            if 0 <= key[0] < len(maze) and 0 <= key[1] < len(maze[0]) and key not in visited:
                if is_connected(maze, row, col, neighbor):
                    node = key
                    # print(key)
                    break
        path += 1
    print(path)
    print(path / 2)
    return visited

def calc_area_size(row,col,maze,visited):
    size = 0
    stack = [(row,col)]
    while stack:
        row, col = stack.pop()
        visited.add((row,col))
        size += 1
        for dir in [(-1,0),(1,0),(0,1),(0,-1)]:
            nrow, ncol = row + dir[0], col + dir[1]
            if (nrow,ncol) in visited:
                continue
            if 0 <= nrow < len(maze) and 0 <= ncol < len(maze[0]) and maze[nrow][ncol] == '.':
                
                stack.append((nrow,ncol))
    return size

def part2(paths, maze):
    pass

def solution():
    filename = "./inputs/q10.txt"
    pipe_maze = [list(line) for line in open(filename).read().split("\n")]
    paths = part1(pipe_maze)
    part2(paths,pipe_maze)
solution()