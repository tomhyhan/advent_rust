from collections import deque
import heapq

def find_pt(row, grid):
    for col in range(len(grid[row])):
        if grid[row][col] == '.':
            return (row,col)

def step_slope(tile, row, col):
    match tile:
        case '^':
            return (row-1, col)
        case '>':
            return (row, col+1)
        case 'v':
            return (row+1, col)
        case '<':
            return (row, col-1)
    print("ERR")
    exit(1)
    
def print_grid(visited, grid):
    for row in range(len(grid)):
        s = ""
        for col in range(len(grid[0])):
            if (row,col) in visited:
                s += "@"
            else:
                s += grid[row][col]
        print(s)

def part1(grid):
    start = find_pt(0, grid)
    end = find_pt(len(grid) - 1, grid)
    visited = {}
    heap = ([(0, start)])
    max_distance = 0

    while heap:
        distance, key = heapq.heappop(heap)

        if key == end:
            print(distance)
            max_distance  = min(max_distance, distance)
            continue
        
        if key in visited and distance < visited[key]:
            continue
        visited[key] = distance
        
        row,col = key
        # print(key)
        for dir in [(0,1),(0,-1),(1,0),(-1,0)]:
            nrow, ncol= row + dir[0], col + dir[1]
            if not(0 <= nrow < len(grid) and 0 <= ncol < len(grid[0])):
                continue 
            tile = grid[nrow][ncol]
            # print(nrow, ncol, tile)
            if tile == '#':
                continue
            heapq.heappush(heap, (distance - 1, (nrow,ncol)))
            # if tile == '.':
            # elif tile in "^>v<":
            #     skey = step_slope(tile,nrow,ncol)
            #     if skey in visited:
            #         continue
            #     new_visited.add(skey)
            #     stack.append((skey, new_visited, distance + 2))
    print("max_distance", max_distance)

def solution():
    filename = "./inputs/q23.txt"
    grid = open(filename).read().split('\n')
    part1(grid)
    pass

solution()

# def part1(grid):
#     start = find_pt(0, grid)
#     end = find_pt(len(grid) - 1, grid)
#     print(start, end)
#     visited = set()
#     visited.add(start)
#     stack = [(start, visited, 0 )]
#     max_distance = 0
#     while stack:
#         key, visited, distance = stack.pop()

#         if key == end:
#             print(distance)
#             max_distance  = max(max_distance, distance)
#             continue
        
#         row,col = key
#         # print(key)
#         for dir in [(0,1),(0,-1),(1,0),(-1,0)]:
#             nrow, ncol= row + dir[0], col + dir[1]
#             if not(0 <= nrow < len(grid) and 0 <= ncol < len(grid[0])):
#                 continue 
#             tile = grid[nrow][ncol]
#             # print(nrow, ncol, tile)
#             if tile == '#' or (nrow,ncol) in visited:
#                 continue
#             new_visited = {v for v in visited} 
#             new_visited.add((nrow,ncol))
#             stack.append(((nrow,ncol), new_visited, distance + 1))
#             # if tile == '.':
#             # elif tile in "^>v<":
#             #     skey = step_slope(tile,nrow,ncol)
#             #     if skey in visited:
#             #         continue
#             #     new_visited.add(skey)
#             #     stack.append((skey, new_visited, distance + 2))
#     print("max_distance", max_distance)
