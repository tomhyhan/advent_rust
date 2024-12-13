vec = [(0,0),(0,4),(-1,4),(-1,0),(0,0)]

l = sum([vec[i][0] * vec[i+1][1] for i in range(len(vec) - 1)])

r = sum([vec[i][1] * vec[i+1][0] for i in range(len(vec) - 1)])

print(abs(l - r) / 2)