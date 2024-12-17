from collections import defaultdict
import heapq

data = open("input/day16.txt").read().split('\n')

grid = []
for line in data:
    grid.append(line.replace('E', '.').replace('S', '.'))
    
n_rows = len(grid)
n_cols = len(grid[0])

srow , scol = n_rows - 2, 1
erow , ecol = 1, n_cols - 2

dirs = [(-1,0),(0,1),(1,0),(0,-1)]
seen = defaultdict(lambda: float("inf"))
stack  = [(0, srow, scol, 1, set())]

# 7036 11048 130536
m = 130536
total_pts = set()
while stack:
    score, row, col, dir, npts = heapq.heappop(stack)
    
    if score > m:
        continue
    
    if row == erow and col == ecol and score == m:
        total_pts |= npts
        continue

    for ndir in range(len(dirs)):
        nrow = row + dirs[ndir][0]
        ncol = col + dirs[ndir][1]
        key = (nrow, ncol, ndir)
        nscore = score + 1 if dir == ndir else score + 1 + 1000
        nnpts = {pt for pt in npts}
        nnpts.add((nrow, ncol))
        if grid[nrow][ncol] != '#' and nscore <= seen[key]:
            seen[key] = nscore
            heapq.heappush(stack, (nscore, nrow, ncol, ndir, nnpts))
            
print(len(total_pts) + 1)
