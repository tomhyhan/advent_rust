from math import sqrt, lcm

def part1(instructions, network):
    current = "AAA"
    ways = 0
    while current != "ZZZ":
        idx = ways % len(instructions)
        current = network[current][0] if instructions[idx] == 'L' else network[current][1]
        ways += 1
    print(ways)

def find_starting_nodes(nodes):
    return [node for node in nodes if node[-1] == 'A']

def is_prime(num):
    if num < 2:
        return False
    for i in range(2, int(sqrt(num))+1):
        if num % i == 0:
            return False
    return True

def common_divisor(num):
    for i in range(2, int(sqrt(num))+1):
        if num % i == 0:
            print(i)

def part2(instructions, network):
    nodes = find_starting_nodes(network.keys())
    zs = []
    for node in nodes:
        current = node
        ways = 0
        while current[-1] != 'Z':
            # print(current)
            idx = ways % len(instructions)
            current = network[current][0] if instructions[idx] == 'L' else network[current][1]
            ways += 1
        zs.append(ways)
    print(lcm(*zs))

def parse(file):
    instructions, mapping = open(file).read().split('\n\n')
    network = {}
    for line in mapping.split('\n'):
        src, dest = line.split(" = (")
        left_des, right_des = dest.strip(')').split(", ")
        network[src] = [left_des, right_des]
    return instructions, network

def gcd(x,y):
    return y if x % y == 0 else gcd(y, x%y)

def solution():
    filename = "./inputs/q8.txt"
    instructions, network = parse(filename)
    # improve with Chinese Remainder Theorem
    part1(instructions, network)
    part2(instructions, network)
    
solution()