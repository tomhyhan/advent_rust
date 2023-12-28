from dataclasses import dataclass

@dataclass
class Range:
    min: int
    max: int

    def __eq__(self, other: object) -> bool:
        return self.max == other.max and self.min == other.min

    def __hash__(self) -> int:
        return hash((self.min, self.max))


class Block:
    def __init__(self, x, y, z):
        self.x = Range(int(x[0]), int(x[1]))
        self.y = Range(int(y[0]), int(y[1]))
        self.z = Range(int(z[0]), int(z[1]))
        self.supports = set()
        self.depends = set()

    def __repr__(self) -> str:
        return f"{self.x} {self.y} {self.z}"

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __hash__(self) -> int:
        return hash((self.x, self.y, self.z))


def parse(filename):
    lines = open(filename).read().split('\n')
    blocks = []
    for line in lines:
        from_, to_ = map(lambda x: x.split(','), line.split("~"))
        block = Block(*list(zip(from_, to_)))
        blocks.append(block)
    return blocks


def find_blocks(z, blocks):
    return [block for block in blocks if block.z.min <= z <= block.z.max]


def can_fit(block, bot_blocks):
    x_y_fit = True
    for bot_block in bot_blocks:
        can_x_fit = bot_block.x.max < block.x.min or bot_block.x.min > block.x.max
        can_y_fit = bot_block.y.max < block.y.min or bot_block.y.min > block.y.max
        if not (can_x_fit or can_y_fit):
            bot_block.supports.add(block)
            block.depends.add(bot_block)
            x_y_fit = False
    return x_y_fit


def move_block(block, blocks):
    while block.z.min > 1:
        bot_z = block.z.min - 1
        bot_blocks = find_blocks(bot_z, blocks)
        if not can_fit(block, bot_blocks):
            break
        block.z.min -= 1
        block.z.max -= 1


def can_disintegrate(block, blocks):
    if len(block.supports) == 0:
        return True

    max_z = block.z.max
    other_blocks = [b for b in find_blocks(max_z, blocks) if b != block]

    if len(other_blocks) == 0:
        return False

    supports = {b for b in block.supports}
    for other_b in other_blocks:
        supports -= other_b.supports
    return len(supports) == 0


def part1(blocks):
    removed = 0
    for block in blocks:
        if can_disintegrate(block, blocks):
            removed += 1
    print(removed)

def count_falls(block, depends):
    falls = 0
    stack = [block]
    while stack:
        cblock = stack.pop()
        for support in cblock.supports:
            skey = str(support)
            dkey = str(cblock)
            if dkey in depends[skey]: 
                depends[skey].remove(dkey) 
                if len(depends[skey]) == 0:
                    falls += 1
                    stack.append(support)
    return falls

def part2(blocks):
    falls = 0
    for block in blocks:
        if not can_disintegrate(block, blocks):
            depends = {str(b): set([str(d) for d in b.depends]) for b in blocks}
            falls += count_falls(block, depends)
    print(falls)

def solution():
    filename = "./inputs/q22.txt"
    blocks = parse(filename)
    blocks.sort(key=lambda x: x.z.min)
    for block in blocks:
        move_block(block, blocks)
    part1(blocks)
    part2(blocks)


solution()
