from copy import deepcopy

def find_start(grid):
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] == 'S':
                return (row,col)

def next_garden_plots(garden_plots, grid, memo):
    next_plots = set() 
    garden_plots = garden_plots - memo["before"]
    while garden_plots:
        row, col = garden_plots.pop()
        for dir in [(0,1),(0,-1),(1,0),(-1,0)]:
            nrow, ncol = row + dir[0], col + dir[1]
            if 0 <= nrow < len(grid) and 0 <= ncol < len(grid[0]):
                if grid[nrow][ncol] != '#':
                    next_plots.add((nrow,ncol))
    next_plots = next_plots | memo["after"]
    return next_plots

def part1(grid):
    steps = 6
    start = find_start(grid)
    garden_plots = set([start])
    for _ in range(steps):
        garden_plots = next_garden_plots(garden_plots, grid)
    print(len(garden_plots))

def print_grid(grid, plots):
    matrix = [list(g) for g in grid]
    for row, col in plots:
        matrix[row][col] = 'O'
    for m in matrix:
        print(''.join(m))

def get_range_plots(garden_plots, row_len, col_len, start):
    r = row_len // 2
    c = col_len // 2
    return len([(row,col) for row, col in garden_plots if start[0] - r //2 <= row <= start[0] + r and start[1] -c <= col <= start[1] + c])

def part2(grid):
    resize = 500
    start = find_start(grid)
    row_len, col_len = len(grid), len(grid[0])
    start = (start[0] + row_len * (resize // 2), start[1] + col_len * (resize // 2))
    grid = [line * resize for line in grid * resize]
    garden_plots = set([start])
    prev = 0
    increases = []
    steps = 350
    # not that necessary
    memo = {"even":{
                "before": set(),
                "after": set(),
            }, 
            "odd": {
                "before": set(),
                "after": set(),
            }}
    for idx in range(steps):
        current_memo = memo["odd"] if idx % 2 == 1 else memo["even"]
        before = deepcopy(garden_plots) 
        garden_plots = next_garden_plots(garden_plots, grid, current_memo)
        after = deepcopy(garden_plots) 
        increases.append(len(garden_plots) - prev)
        prev = len(garden_plots)
        print(idx, len(garden_plots))
        if idx % 2 == 1:
            memo["odd"]["before"] = before
            memo["odd"]["after"] = after
        else:
            memo["even"]["before"] = before
            memo["even"]["after"] = after
    print("length", len(garden_plots))

    for i in range(5, len(increases)):
        pairsize = i
        for start in range(len(increases) - pairsize * 3):
            end = start + pairsize
            pair = increases[start : end]
            next_pair = increases[end: end + pairsize]
            next_next_pair = increases[end + pairsize: end + pairsize*2]
            found = True
            for j in range(len(pair)):
                if next_pair[j] - pair[j] != next_next_pair[j] - next_pair[j]:
                    found = False    
                    break
            if found:
                print("found", pairsize, start, pair, next_pair)
                break

# sample 
# found 11 37 [48, 50, 45, 64, 54, 39, 50, 60, 68, 59, 81] [64, 66, 59, 82, 70, 48, 61, 74, 84, 73, 99]

# real at steps 700
# found 131 1 [4, 8, 6, 11, 10, 16, 9, 20, 17, 21, 16, 22, 15, 26, 22, 41, 23, 44, 21, 50, 21, 61, 18, 68, 11, 77, 9, 92, 9, 87, 11, 93, 11, 97, 5, 107, 13, 122, 23, 128, 16, 135, 14, 130, 21, 140, 24, 144, 25, 144, 19, 151, 24, 151, 25, 155, 30, 177, 33, 175, 70, 192, 78, 183, 81, 187, 85, 153, 81, 152, 92, 156, 106, 155, 95, 157, 103, 174, 91, 178, 83, 179, 105, 186, 107, 185, 116, 200, 104, 205, 94, 203, 114, 213, 94, 225, 91, 236, 106, 237, 107, 245, 111, 240, 112, 243, 119, 237, 140, 242, 140, 232, 156, 214, 176, 203, 191, 212, 195, 202, 205, 214, 208, 192, 220, 215, 217, 217, 215, 217, 216] [239, 222, 237, 206, 238, 224, 230, 249, 236, 242, 224, 240, 224, 261, 208, 308, 193, 324, 181, 338, 163, 366, 148, 372, 131, 393, 133, 425, 123, 404, 116, 419, 115, 407, 108, 420, 130, 457, 144, 454, 144, 456, 132, 442, 145, 466, 138, 471, 138, 468, 135, 462, 155, 453, 152, 468, 156, 516, 145, 505, 221, 573, 246, 550, 239, 553, 243, 481, 191, 470, 215, 471, 236, 464, 207, 473, 225, 501, 202, 498, 189, 496, 238, 503, 224, 500, 247, 529, 225, 528, 202, 521, 241, 522, 198, 543, 188, 559, 220, 570, 214, 581, 227, 546, 233, 550, 257, 530, 299, 535, 303, 512, 332, 460, 367, 432, 395, 442, 402, 426, 419, 448, 430, 398, 446, 438, 438, 437, 431, 433, 429]

def solution():
    filename = "./inputs/q21.txt"
    grid = open(filename).read().split('\n')
    # part1(grid)
    # part2(grid)
    print(divmod((26501365-1 - 0 ), 131))
    print(91055 - 32896)
    print(32896 - 3725)
    print(58159 - 29171)
    total = 3725
    for i in range(202300):
        total += 29171 + 28988 * i
    print(total)
# 846 1464 2244 3186 4290
# 894 1528 2324
# 3725 32896 91055
solution()

