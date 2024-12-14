data = open("input/day13.txt").read().split("\n\n")

total = 0
for d in data:
    d = d.split('\n')
    AX, AY = d[0].split(':')[1].strip().split(',')
    AX, AY = int(AX.split('+')[1]), int(AY.split('+')[1])
    BX, BY = d[1].split(':')[1].strip().split(',')
    BX, BY = int(BX.split('+')[1]), int(BY.split('+')[1])
    PX, PY = d[2].split(':')[1].strip().split(',')
    PX, PY = int(PX.split('=')[1]), int(PY.split('=')[1])
    # print(AX, AY, BX, BY, PX, PY)
    # min_token = float('inf')
    # for i in range(101):
    #     for j in range(101):
    #         if AX * i + BX * j == PX and AY * i + BY * j == PY:
    #             min_token = min(min_token, i*3 + j)
    # if min_token != float("inf"):
    #     total += min_token

print(total)
print(10000000008400 // 94)
