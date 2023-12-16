from collections import OrderedDict

class Node:
    def __init__(self, value):
        self.value = value
        self.prev = None
        self.next = None

    def __repr__(self):
        return f"{self.value}"
    
class BoxInfo:
    def __init__(self):
        self.current = None
        self.head = None
        self.tracker = {}

    def store(self, label, value):
        # update
        if label in self.tracker:
            self.tracker[label].value = value
            return

        # add
        node = Node(value)
        if self.head is None:
            self.head = node 
            self.current = node
        else:
            print(self.current)
            node.prev = self.current
            self.next = node
            self.current = node
        print(node.prev)
        print("head", self.head.next)
        self.tracker[label] = node
        # print(self.current)
    
    def remove(self,label):
        if label not in self.tracker:
            return 

        removed = self.tracker[label]
        if self.current == removed:
            self.current = removed.prev
        if removed == self.head:
            self.head = removed.next
        if removed.prev:
            removed.prev.next = removed.next
        if removed.next:
            removed.next.prev =  removed.prev

    def __repr__(self):
        return f"{self.tracker} {self.head}"
    
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
    boxes = [BoxInfo() for i in range(256)]
    for string in strings:
        if '=' in string:
            label, focal = string.split('=')
            hashed = hash(label)
            boxes[hashed].store(label, int(focal))
        else:
            label, _ = string.split('-')
            hashed = hash(label)
            boxes[hashed].remove(label)
    
    # print(boxes[0])
    print(boxes[3].tracker["pc"].next)
    
    # total = 0
    # for box_num, box in enumerate(boxes):
    #     current = box
    #     for idx, (label, focal) in enumerate(box.items()):
    #         slot = idx + 1
    #         box_len = box_num + 1
    #         # print(slot, box_len, focal)
    #         total += calc_power(slot, box_len, focal)
    # print(total)

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