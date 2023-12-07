# des src range
class Fertilizer:
    def __init__(self, src_range, des_range, diff):
        self.src_range = src_range
        self.des_range = des_range
        self.diff = diff

    def __repr__(self):
        return f"{self.src_range} {self.des_range} {self.diff}"        
        
def is_within_range(fertilizers, seed):
    for f in fertilizers:
        if f.src_range[0] <= seed <= f.src_range[1]:
            return seed - f.diff
    return False

def part1(seeds, mappings):
    min_location = float("inf")
    for seed in seeds:
        for fertilizers in mappings:
            if (new_seed := is_within_range(fertilizers, seed)):
                seed = new_seed
        min_location = min(min_location, seed)
    print("min_location", min_location)

def part2(seeds, mappings):
    stack = []
    locations = []
    for i in range(0,len(seeds),2):
        src, range_ = seeds[i], seeds[i+1]
        stack.append((src, src + range_ - 1, 0))
    while stack:
        seed_start, seed_end, map_idx = stack.pop()
        # print(seed_start, seed_end, map_idx)
        if map_idx == 7:
            locations.append(seed_start)
            continue
        fertilizers  = mappings[map_idx]
        found = False
        for f in fertilizers:
            if f.src_range[0] <= seed_start <= f.src_range[1]:
                if seed_end > f.src_range[1]:
                    stack.append((seed_start - f.diff, f.src_range[1] - f.diff, map_idx + 1))
                    stack.append((f.src_range[1] + 1, seed_end, map_idx))
                else:
                    stack.append((seed_start - f.diff, seed_end - f.diff, map_idx + 1))
                found = True
                break
        if not found:
            stack.append((seed_start, seed_end, map_idx + 1))
    print(min(locations))

def parse_lnput(file):
    blocks = iter(file.split("\n\n"))
    seeds = list(map(int, next(blocks).split(": ")[1].split()))
    mappings = []
    for block in blocks:
        lines = iter(block.split('\n'))
        fertilizers = []
        for line in lines:
            if 'a' <= line[0] <='z':
                continue
            des, src, range_ = map(int, line.split())
            des_range = [des, des + range_ - 1]
            src_range = [src, src + range_ - 1]
            fertilizers.append(Fertilizer(src_range, des_range, src - des))
        mappings.append(fertilizers)
    return seeds, mappings

def solution():
    filename = "./inputs/q5.txt"
    file = open(filename).read()
    seeds, fertilizers = parse_lnput(file)
    part1(seeds, fertilizers)
    part2(seeds, fertilizers)


solution()