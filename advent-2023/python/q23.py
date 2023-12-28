from collections import deque, defaultdict

MIN = float('-inf')

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
                # s += "@"
                s += str(visited[(row,col)]) + " " if visited[(row,col)] < 10 else str(visited[(row,col)])
            else:
                s += grid[row][col] + " "
        print(s)

def part1(grid):
    start = find_pt(0, grid)
    end = find_pt(len(grid) - 1, grid)
    print(start, end)
    visited = set()
    visited.add(start)
    stack = [(start, visited, 0 )]
    max_distance = 0
    while stack:
        key, visited, distance = stack.pop()

        if key == end:
            print(distance)
            max_distance  = max(max_distance, distance)
            continue
        
        row,col = key
        # print(key)
        for dir in [(0,1),(0,-1),(1,0),(-1,0)]:
            nrow, ncol= row + dir[0], col + dir[1]
            if not(0 <= nrow < len(grid) and 0 <= ncol < len(grid[0])):
                continue 
            tile = grid[nrow][ncol]
            # print(nrow, ncol, tile)
            if tile == '#' or (nrow,ncol) in visited:
                continue
            new_visited = {v for v in visited} 
            new_visited.add((nrow,ncol))
            stack.append(((nrow,ncol), new_visited, distance + 1))
            # if tile == '.':
            # elif tile in "^>v<":
            #     skey = step_slope(tile,nrow,ncol)
            #     if skey in visited:
            #         continue
            #     new_visited.add(skey)
            #     stack.append((skey, new_visited, distance + 2))
    print("max_distance", max_distance)

def can_move(row,col,grid):
    if not(0 <= row < len(grid) and 0 <= col < len(grid[0])) or grid[row][col] == '#':
        return False
    return True

def find_grounds(grid, start, end):
    grounds = []
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] == '#':
                continue
            opens = find_open_spaces(row,col,grid)
            if opens > 2 or (row,col) in[start,end]:
                grounds.append((row,col)) 
    return grounds

def prune(grid, start, end):
    grounds = find_grounds(grid, start, end)
    g_distances = defaultdict(list) 
    for ground in grounds:
        stack = [(0, ground)]
        visited = set()
        visited.add(ground)
        while stack:
            distance, key = stack.pop()
            row, col = key
            for dir in [(0,1),(0,-1),(1,0),(-1,0)]:
                nrow, ncol= row + dir[0], col + dir[1]
                if not can_move(nrow,ncol,grid):
                    continue
                if (nrow,ncol) in visited:
                    continue
                visited.add((nrow,ncol))
                if (nrow,ncol) in grounds:
                    g_distances[ground].append(((nrow,ncol), distance + 1))
                else:
                    stack.append((distance + 1, (nrow,ncol)))
    return g_distances

def part2(grid):
    start = find_pt(0, grid)
    end = find_pt(len(grid) - 1, grid)
    grounds = prune(grid, start, end)
    stack = [(0, start, set())]
    max_distance = 0
    while stack:
        distance, key, visited = stack.pop()

        if key == end:
            max_distance = max(max_distance, distance)

        if key in visited:
            continue
        visited.add(key)

        for ground in grounds[key]:
            new_key, t_distance = ground
            new_visitied = {v for v in visited}
            stack.append((distance + t_distance, new_key, new_visitied))
    print(max_distance)
    
def find_open_spaces(row,col,grid):
    opens = 0
    for dir in [(0,1),(0,-1),(1,0),(-1,0)]:
        nrow, ncol= row + dir[0], col + dir[1]
        if not can_move(nrow, ncol, grid):
                continue
        opens += 1
    return opens

def find_ways(row,col,grid):
    way_ins = []
    way_outs = []
    for dir in [(0,1,'>'),(0,-1,'<'),(1,0,'v'),(-1,0,'^')]:
        nrow, ncol= row + dir[0], col + dir[1]
        symbol = dir[2]
        if not can_move(nrow, ncol, grid):
                continue
        if grid[nrow][ncol] == symbol:
            way_outs.append((nrow,ncol))
        else:
            way_ins.append((nrow,ncol))
            
    return way_ins, way_outs

def solution():
    filename = "./inputs/q23.txt"
    grid = open(filename).read().split('\n')
    # part1(grid)
    part2(grid)
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

#  start = find_pt(0, grid)
#     end = find_pt(len(grid) - 1, grid)
#     visited = defaultdict(lambda: MIN)
    
#     stack = [(0, start)]
#     while stack:
#         distance, key = stack.pop()

#         if key == end:
#             print(distance)
#             print_grid(visited, grid )
#             break

#         if visited[key] != MIN:
#             continue
#         visited[key] = distance

#         row, col = key
#         for dir in [(0,1),(0,-1),(1,0),(-1,0)]:
#             nrow, ncol= row + dir[0], col + dir[1]
#             if not can_move(nrow, ncol, grid):
#                 continue
#             opens = find_open_spaces(nrow,ncol,grid)
#             if opens <= 2:
#                 stack.append((distance + 1, (nrow,ncol)))
#                 pass
#             else:
#                 way_ins, way_outs = find_ways(nrow,ncol,grid)
#                 all_discovers = True
#                 distance_ins = MIN
#                 for way_in in way_ins:
#                     if visited[way_in] == MIN:
#                         all_discovers = False
#                     distance_ins = max(distance_ins, visited[way_in])
#                 if all_discovers:
#                     stack.append((distance_ins + 1, (nrow,ncol)))