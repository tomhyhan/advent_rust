import collections
import math
import re
import sys

# import sortedcollections

with open(sys.argv[1]) as f:
    lines = [l.rstrip('\n') for l in f]
    lines = [[i for i in re.findall(r'-?\d+|[A-Z]+', l)] for l in lines]

    outs = {}
    for line in lines:
        co, cq = line[-2:]
        co = int(co)

        ins = []
        for a, b in zip(line[:-2][::2], line[:-2][1::2]):
            ins.append((int(a), b))
        assert cq not in outs
        outs[cq] = (co, ins)
    print(outs)


    def tryfuel(fuel):
        need = {'FUEL': fuel}
        leftovers = collections.defaultdict(int)
        while True:
            try:
                nk = next(n for n in need if n != 'ORE')
            except StopIteration:
                break
            quant, ins = outs[nk]

            # For part 1, I did it the lazy way (one reaction at a time):
            #
            # if need[nk] < quant:
            #     leftovers[nk] += quant - need[nk]
            #     del need[nk]
            # elif need[nk] == quant:
            #     del need[nk]
            # else:
            #     need[nk] -= quant

            # For part 2, more efficient:
            # print(fnk {nk} need[nk] {need[nk]} quant {quant})
            # 1 []
            # need / 1
            d, m = divmod(need[nk], quant)
            # print(d,m )
            if m == 0:
                del need[nk]
            else:
                del need[nk]
                leftovers[nk] = quant - m
                d += 1
            # print(fd {d} m {m})
            # print(nk)
            # print(leftovers)
            # print(f{ins})
            # nk = FEUL
            for a, b in ins:
                need[b] = need.get(b, 0) + d * a - leftovers[b]
                    
                del leftovers[b]
        return need['ORE']

    # part 1
    # (I hadn't extracted it to a function during the contest, but otherwise
    # this is what I wrote)
    print(tryfuel(1))

    # part 2
    # I didn't feel like writing a binary search so I literally tried different
    # numbers while rerunning this program and did a binary-esque search by hand:
    # print(10 ** 12)
    # print(tryfuel(82892753))

    # part 2 appendix: probably should've done it like this in the first place
    a, b = 1, 2
    while tryfuel(b) < 10**12:
        a, b = b, b * 2
    print("a", a,b)
    while b - a >= 2:
        half = a + (b - a) // 2
        if tryfuel(half) > 10**12:
            b = half
        else:
            a = half
    print(a)