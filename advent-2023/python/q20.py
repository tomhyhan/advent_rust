class FlipFlob:
    def __init__(self, src, dest):
        self.status = False
        self.src = src
        self.dest = dest

    def __repr__(self):
        return f"FLIP {self.src} {self.dest}"


class Conjunction:
    def __init__(self, src, modules):
        self.src = src
        self.dest = modules

    def __repr__(self):
        return f"CON {self.src} {self.dest}"


def part1():
    pass


def parse(filename):
    lines = open(filename).read().split('\n')
    circuits = {}
    for line in lines:
        src, dest = line.split(" -> ")
        dest = dest.split(', ')
        src = src[1:]
        if line.startswith('broadcaster'):
            circuits['broadcaster'] = dest
        elif line.startswith('%'):
            circuits[src] = FlipFlob(src, dest)
        elif line.startswith('&'):
            circuits[src] = Conjunction(src, dest)
    print(circuits)


def solution():
    filename = "./inputs/q20.txt"
    parse(filename)
    part1()
    pass


solution()

# % (ff) -> default: off
#    -> high: nothing
#    -> low: if OFF, turns ON sends high
#            if ON, turns OFF sends low

# & (con): connected modules' type
#       -> default: low pulse
#       -> if all highs, sends low
#       -> else, sends high
