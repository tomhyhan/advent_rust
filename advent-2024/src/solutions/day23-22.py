from collections import defaultdict


def bron_kerbosch(graph, r=None, p=None, x=None):
    if r is None:
        r = set()
        p = set(graph.keys())
        x = set()

    if not p and not x:
        print("Found max clique: ", r)
        return

    pivot = next(iter(p | x))
    for v in (p - set(graph[pivot])):
        n = set(graph[v])
        bron_kerbosch(
            graph,
            r=r.union({v}),
            p=p & n,
            x=x & n
        )
        p -= {v}
        x |= {v}


data = open("input/day23.txt").read().splitlines()


graph = defaultdict(list)

for line in data:
    f, s = line.split('-', 1)
    graph[f].append(s)
    graph[s].append(f)

bron_kerbosch(graph)
