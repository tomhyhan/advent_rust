from dataclasses import dataclass
from collections import defaultdict

# how to find bottom z occupency?

@dataclass
class Range:
    min : int
    max : int
 
class Block:
    def __init__(self, x, y, z):
        self.x = Range(int(x[0]), int(x[1]))
        self.y = Range(int(y[0]), int(y[1]))
        self.z = Range(int(z[0]), int(z[1]))
        self.depends = []
        
    def __repr__(self) -> str:
        return f"{self.x} {self.y} {self.z}"

    def __eq__(self):
        pass

def parse(filename):
    lines = open(filename).read().split('\n')
    blocks = []
    for line in lines: 
        from_, to_ = map(lambda x: x.split(','), line.split("~"))
        block = Block(*list(zip(from_, to_)))
        blocks.append(block)
    return blocks

def find_bot_blocks(bot_z, blocks):
    return [block for block in blocks if block.z.min <= bot_z <= block.z.max]

def can_fit(block, bot_blocks):
    x_y_fit = True
    # print("bot_blocks", bot_blocks)
    # print(block)
    for bot_block in bot_blocks:
        can_x_fit = bot_block.x.max < block.x.min or bot_block.x.min > block.x.max
        can_y_fit = bot_block.y.max < block.y.min or bot_block.y.min > block.y.max
        if not (can_x_fit or can_y_fit):
            bot_block.depends.append(block)
            x_y_fit = False
    return x_y_fit

def move_block(block, blocks):
    while block.z.min > 1:
        bot_z = block.z.min - 1
        # print(bot_z)
        bot_blocks = find_bot_blocks(bot_z,blocks)
        # print(bot_blocks)
        if not can_fit(block, bot_blocks):
            break
        block.z.min -= 1
        block.z.max -= 1

def part1(blocks):
    blocks.sort(key=lambda x: x.z.min)
    for block in blocks:
        move_block(block, blocks)
    # move_block(blocks[2], blocks)
    # print("---")
    for block in blocks:
        print(block)
        # print(block.depends)
        
def solution():
    filename = "./inputs/q22.txt"
    blocks = parse(filename)
    part1(blocks)
    pass

solution()