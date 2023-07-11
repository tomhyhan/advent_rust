import math
import sys

# $ python 10.py 10.in
# (part1 answer)
# (part2 answer)
G = []
for line in open(sys.argv[1]).readlines():
    line = line.strip()
    G.append(str(line))
R = len(G)
C = len(G[0])

def gcd(x,y):
    return y if x==0 else gcd(y%x, x)

ans = (0,0,0,set())
for r in range(R):
    for c in range(C):
        if G[r][c] != '#':
            continue
        seen = set()
        for rr in range(R):
            for cc in range(C):
                if G[rr][cc] == '#' and (rr!=r or cc!=c):
                    dr = rr-r # 2 - 4
                    dc = cc-c # 3 - 3 
                    g = gcd(dr,dc) # 2 0
                    if g < 0:
                        g *= -1
                    seen.add((-dr//g, dc//g))
        if len(seen) > ans[0]:
            ans = (len(seen),r,c,seen)

ans,r,c,seen = ans
# print(ans)
# print(r,c)
# print(seen)
# x = -1
# y = -1
# print(math.atan2(y,x))
# print(math.pi/2.0)
# print('gcd', gcd(-2,0))
# print(2//2, 0//2)
# to_sort = []
# for (dr,dc) in seen:
#     key = math.atan2(dr,dc)
#     print('key', key)
#     if key > math.pi/2.0:
#         key -= 2*math.pi
#     to_sort.append((key, (dr,dc)))
# to_sort = list(reversed(sorted(to_sort)))
# print(to_sort)
print("ans", ans)
print(seen)
to_sort = []
for (dr,dc) in seen:
    key = math.atan2(dr,dc)
    if key > math.pi/2.0:
        key -= 2*math.pi
    to_sort.append((key, (dr,dc)))
to_sort = list(reversed(sorted(to_sort)))
print(to_sort)
print(to_sort[199])
print(math.pi)
winner = to_sort[199][1]
rr = r-winner[0]
cc = c+winner[1]
while G[rr][cc]!='#':
    rr-=winner[0]
    cc+=winner[1]
assert G[rr][cc]=='#'
print(cc*100+rr)

# print(math.atan2(y,x))