import heapq
from collections import defaultdict

data = open("input/day9.txt").read()

print(len(data))
disk = []


class Node:
    def __init__(self, id, index, space):
        self.id = id
        self.index = index
        self.space = space
        self.prev, self.next = None, None

    def __lt__(self, other):
        return self.index < other.index

    def __repr__(self):
        return f"id: {self.id} index: {self.index} space:{self.space}"


class LinkedList:
    def __init__(self):
        self.head, self.tail = None, None

    def insert_node(self, node):
        if self.head is None:
            self.head = node
            self.tail = node
            return

        self.tail.next = node
        node.prev = self.tail
        self.tail = node


linked_list = LinkedList()

index = 0
free_spaces = defaultdict(list)

for i, num in enumerate(data):
    num = int(num)
    if i % 2 == 0:
        linked_list.insert_node(
            Node(id=i//2, index=index, space=num)
        )
    else:
        free_node = Node(id=-1, index=index, space=num)
        linked_list.insert_node(free_node)
        heapq.heappush(free_spaces[num], free_node)
    index += num

current = linked_list.tail
while current:
    if current.id == -1:
        current = current.prev
        continue
    free = None
    curr_space = 0
    for space in range(current.space, 10):
        if free_spaces[space]:
            if not free:
                free = free_spaces[space][0]
                curr_space = space
                continue
            if free_spaces[space][0].index < free.index:
                free = free_spaces[space][0]
                curr_space = space
    if not free:
        current = current.prev
        continue

    if free.index < current.index:
        free = heapq.heappop(free_spaces[curr_space])
        if free.space == current.space:
            free.id = current.id
        else:
            new_node = Node(
                id=current.id,
                space=current.space,
                index=free.index
            )
            new_node.prev = free.prev
            new_node.next = free
            free.prev.next = new_node
            free.prev = new_node
            free.space -= current.space
            free.index += current.space
            heapq.heappush(free_spaces[free.space], free)
        current.id = -1
    current = current.prev

current = linked_list.head
total = 0
while current:
    if current.id != -1:
        for i in range(current.index, current.index + current.space):
            total += i * current.id
    current = current.next
print(total)
