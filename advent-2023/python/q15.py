from collections import OrderedDict

class Node:
    def __init__(self, value):
        self.value = value
        self.prev = self.next = None

class BoxInfo:
    def __init__(self):
        self.current = None
        self.tracker = {}

    def store(self, label, value):
        # update
        if label in self.tracker:
            self.tracker[label].value = value
            return

        # add
        node = Node(value)
        if self.head is None:
            self.head = 
            

def hash(string):
    value = 0
    for c in string:
        value += ord(c)
        value *= 17
        value %= 256
    return value

def calc_power(slot, box_len, focal):
    return slot * box_len * focal
    pass

def part1(strings):
    return sum([hash(string) for string in strings])

def part2(strings):
    # ordered-dict, hash + linkedlist, list
    boxes = [OrderedDict() for i in range(256)]
    for string in strings:
        if '=' in string:
            label, focal = string.split('=')
            hashed = hash(label)
            boxes[hashed][label] = int(focal)
        else:
            label, _ = string.split('-')
            hashed = hash(label)
            if label in boxes[hashed]: 
                del boxes[hashed][label]
    
    total = 0
    for box_num, box in enumerate(boxes):
        for idx, (label, focal) in enumerate(box.items()):
            slot = idx + 1
            box_len = box_num + 1
            # print(slot, box_len, focal)
            total += calc_power(slot, box_len, focal)
    print(total)

def solution():
    filename = "./inputs/q15.txt"
    strings = open(filename).read().split(',')
    print(part1(strings))
    part2(strings)
solution()


# def part2(strings):
#     # ordered-dict
#     boxes = [OrderedDict() for i in range(256)]
#     for string in strings:
#         if '=' in string:
#             label, focal = string.split('=')
#             hashed = hash(label)
#             boxes[hashed][label] = int(focal)
#         else:
#             label, _ = string.split('-')
#             hashed = hash(label)
#             if label in boxes[hashed]: 
#                 del boxes[hashed][label]
    
#     total = 0
#     for box_num, box in enumerate(boxes):
#         for idx, (label, focal) in enumerate(box.items()):
#             slot = idx + 1
#             box_len = box_num + 1
#             # print(slot, box_len, focal)
#             total += calc_power(slot, box_len, focal)
#     print(total)