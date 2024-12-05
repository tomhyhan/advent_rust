d = {
    'a': 2,
    'b': 1,
    'c': 0,
}

print(sorted(d.keys(), key=lambda x: d[x]))