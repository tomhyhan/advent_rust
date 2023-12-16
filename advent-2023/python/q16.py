DIRECTIONS = [(-1,0),(0,1),(1,0),(0,-1)]

def get_next_move(row, col, dir, grid):
    head = DIRECTIONS[dir]
    nrow, ncol = row + head[0], col + head[1]
    if not(0 <= nrow < len(grid) and 0 <= ncol < len(grid[0])):
        return -1,-1,-1
    current = grid[nrow][ncol]
    match current:
        case '-':
            if dir == 0 or dir == 2:
                yield nrow,ncol,(dir+1)%4
                yield nrow,ncol,(dir+3)%4
            else:
                yield nrow,ncol,dir
        case '\\':
            if dir == 1 or dir == 3:
                yield nrow,ncol,(dir+1)%4
            else:
                yield nrow,ncol,(dir+3)%4
        case '/':
            if dir == 1 or dir == 3:
                yield nrow,ncol,(dir+3)%4
            else:
                yield nrow,ncol,(dir+1)%4
        case '|':
            if dir == 1 or dir == 3:
                yield nrow,ncol,(dir+1)%4
                yield nrow,ncol,(dir+3)%4
            else:
                yield nrow,ncol,dir
        case '.':
            yield nrow,ncol,dir

def set_directions(row, col, grid):
    # 0 - n, 1 - e, 2 - s , 3 - w
    is_up_border = row == 0
    is_down_border = row == len(grid) -1
    is_left_border = col == 0
    is_right_border = col == len(grid[0]) - 1

    if is_up_border:
        yield (row-1,col,2)
    if is_right_border:
        yield (row,col+1,3)
    if is_down_border:
        yield (row+1,col,0)
    if is_left_border:
        yield (row,col-1,1)


def analyze_energized_path(start, grid):
    moves = [start]
    visited = set()
    while moves:
        row, col, dir = moves.pop()
        for nrow, ncol, ndir in get_next_move(row, col, dir, grid):
            if ndir == -1:
                continue
            if (nrow, ncol, ndir) in visited:
                continue
            visited.add((nrow, ncol, ndir))
            moves.append((nrow, ncol, ndir))
    energized = set([(r,c) for r,c,d in visited])
    return len(energized)
    
def part1(grid):
    start = (0,-1,1)
    energized = analyze_energized_path(start, grid)
    print(energized)

def part2(grid):
    max_energized = 0
    for erow in range(len(grid)):
        for ecol in range(len(grid[0])):
            is_row_border = erow == 0 or erow == len(grid) -1
            is_col_border = ecol == 0 or ecol == len(grid[0]) - 1
            is_border = is_row_border or is_col_border
            if not is_border:
                continue
            for start in set_directions(erow, ecol, grid):
                energized = analyze_energized_path(start, grid)
                max_energized = max(max_energized, energized)
    print(max_energized)    

def solution():
    filename = "./inputs/q16.txt"
    grid = [list(line) for line in open(filename).read().split('\n')]
    # print(grid[1][4] == "\\")
    part1(grid)
    part2(grid)
    pass

solution()