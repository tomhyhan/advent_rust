from collections import defaultdict
import networkx as nx

def part1(components):
    G = nx.DiGraph()
    for com, cons in components.items():
        for con in cons:
            G.add_edge(com, con, capacity=1.0)
    
    for com, cons in components.items():
        for con in cons:
            cut_value, partition = nx.minimum_cut(G, com, con)
            if cut_value == 3:
                print(com,con)
                print(partition)
                print(len(partition[0]) * len(partition[1]))
                
    
    
def parse(filename):
    components = defaultdict(list)
    for line in open(filename).read().split('\n'):
        com, cons = line.split(': ')
        for con in cons.split():
            components[com].append(con)
            components[con].append(com)
    return components

def solution():
    filename = "./inputs/q25.txt"
    components = parse(filename)
    part1(components)
    pass

solution()