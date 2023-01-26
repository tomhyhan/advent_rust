import re

lines = open("input.txt").readlines()
p1_total = 0
for line in lines:
    p1_total += 2
    p1_total += len(re.findall("\\\[\"\\\]", line))
    p1_total += 3 * len(re.findall("\\\[x][0-9a-f]{2}", line))
    print(re.findall("\\\[x][0-9a-f]{2}", line))
    print(re.findall("\\\[\"\\\]", line))
print(p1_total )