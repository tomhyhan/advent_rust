def combinations_with_replacement(iterable, r):
    # combinations_with_replacement('ABC', 2) --> AA AB AC BB BC CC
    pool = tuple(iterable)
    # print(pool)
    n = len(pool)
    if not n and r:
        return
    indices = [0] * r
    # print(indices)
    # for i in indices:
    #   print(i)
    yield tuple(pool[i] for i in indices)
    # print("as")
    while True:
        print('sdfdf')
        for i in reversed(range(r)):
            # print(list(reversed(range(r))))
            # i = 1, 0 , n = 3
            print("dfdf")
            print(f"current idx - {i}")
            if indices[i] != n - 1:
                break
        else:
            print("as")
            return
        print(i)
        print(indices)
        indices[i:] = [indices[i] + 1] * (r - i)
        print(indices)
        print()
        yield tuple(pool[i] for i in indices)
        
x = combinations_with_replacement("ABC", 2)
print(x.__next__())
print(x.__next__())
print(x.__next__())
print(x.__next__())
print(x.__next__())
print(x.__next__())
print(x.__next__())
# for x in combinations_with_replacement("ABC", 2):
#   print(x)