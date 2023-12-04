from collections import defaultdict

def is_valid_partnum(row,col,grid):
    for i in range(-1,2):
        for j in range(-1,2):
            nrow, ncol = row + i, col + j
            if nrow == row and ncol == col:
                continue
            if 0 <= nrow < len(grid) and 0 <= ncol < len(grid[0]):
                current = grid[nrow][ncol]
                if current != '.' and not current.isdigit():
                    return (nrow,ncol) 
    return None

def find_partnum(row,col,grid,visited):
    valid_partnum = False
    stack = [(row,col)]
    partnum = ""
    star_location = None
    while stack:
        crow, ccol = stack.pop()
        if ccol >= len(grid[0]):
            break
        
        current = grid[crow][ccol]

        if not current.isdigit():
            break

        visited.add((crow,ccol))
        if not valid_partnum and (symbol_location:=is_valid_partnum(crow,ccol,grid)):
            valid_partnum = True
            srow, scol = symbol_location
            if grid[srow][scol] == '*':
                star_location = (srow,scol) 

        partnum += current
        stack.append((crow,ccol+1))

    return (partnum, star_location) if valid_partnum else (None, None)

def part1(grid):
    visited = set()
    result = 0
    gear_ratio = 0 
    stars = defaultdict(list)
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if (row,col) in visited:
               continue
            current = grid[row][col]
            if current.isdigit() and (partnum_data:= find_partnum(row,col,grid,visited)):
                partnum, star_location = partnum_data
                if partnum is None:
                    continue
                result += int(partnum)
                if star_location is not None:
                    stars[star_location].append(partnum)
    print(result)

    for partnum_list in stars.values():
        if len(partnum_list) != 2:
            continue
        gear_ratio += (int(partnum_list[0]) * int(partnum_list[1]))
    print(gear_ratio)

def main():
    filename = "./inputs/q3.txt"
    grid = [list(line)  for line in open(filename).read().split('\n')]
    part1(grid)
    # improve with regex
    # save all sumbol pos -> iter rows -> add each partnum to symbols 
    # x = "467..114.."
    # for n in re.finditer(r'\d+', x):
    #     print(n)
    pass

main()