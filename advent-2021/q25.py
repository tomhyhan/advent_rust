def print_grid(grid):
    for line in grid:
        print("".join(line))
    print()

def move_cucumbers_right(grid):
    row_len = len(grid)
    col_len = len(grid[0])
    marked = set()

    did_change = False
    for row in range(row_len):
        for col in range(col_len):
            next_col = (col + 1) % col_len 
            if (row,col) in marked or (row,next_col) in marked:
                continue
            if grid[row][col] == ">" and grid[row][next_col] == ".":
                grid[row][col], grid[row][next_col] = grid[row][next_col], grid[row][col]
                marked.add((row, next_col))
                marked.add((row, col))
                did_change = True
    return did_change

def move_cucumbers_down(grid):
    row_len = len(grid)
    col_len = len(grid[0])
    marked = set()

    did_change = False
    for row in range(row_len):
        for col in range(col_len):
            next_row = (row + 1) % row_len 
            if (row,col) in marked or (next_row, col) in marked:
                continue
            if grid[row][col] == "v" and grid[next_row][col] == ".":
                grid[row][col], grid[next_row][col] = grid[next_row][col], grid[row][col]
                marked.add((next_row, col))
                marked.add((row, col))
                did_change = True
    return did_change
    
def solution():
    lines = open("./inputs/q25.txt").read().strip()
    grid = []
    for line in lines.split('\n'):
        grid.append(list(line))

    cnt = 0
    moves = True
    while moves:
        moves = False
        if move_cucumbers_right(grid):
            moves = True
        if move_cucumbers_down(grid):
            moves = True
        cnt += 1
    print(cnt)

solution()

def test():
    x = {1:1, 2:2 , 3:3}
    y = {100 if k == 1 else k:v for k, v in x.items()}
    print(y)
