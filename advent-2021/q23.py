import heapq
from copy import deepcopy

FORBIDDEN_POSITIONS = {'A':2, 'B':4, 'C':6, 'D':8 }
POD_USED_ENERGIES = {'A':1, 'B':10, 'C':100, 'D':1000 }
class HallWay:
    def __init__(self, map):
        self = map

    def __repr__(self):
        return f"{self}"

class Rooms:
    def __init__(self, arr):
        self = arr
        
    def __repr__(self):
        return f"{self}"

    def __lt__(self, other):
        return False

def parse_input(filename):
    lines = open(filename).read().split('\n')
    # hallway = HallWay(list(lines[1].strip('#')))
    arrangements = [[char for char in line if 'A' <= char <= 'D']for line in lines[2:4]]
    # rooms = Rooms(list(map(list, zip(*arrangements))))
    # return (rooms, hallway)
    return (list(lines[1].strip('#')), list(map(list, zip(*arrangements))))
def next_room_moves(state):
    rooms, hallway = state

    next_moves = []
    for room_num, room in enumerate(rooms):
        if not room:
            continue
        
        room_amphipod = "ABCD"[room_num]
        
        if all(amphipod == room_amphipod for amphipod in room):
            continue 
        
        steps = (2 - len(room)) + 1

        to_hallways = []

        left = FORBIDDEN_POSITIONS[room_amphipod] -1
        left_steps = steps
        while left >= 0:
            if hallway[left] != '.':
                break
            left_steps += 1
            if left not in FORBIDDEN_POSITIONS.values():
                to_hallways.append([left_steps, left])
            left -= 1

        right = FORBIDDEN_POSITIONS[room_amphipod] +1
        right_steps = steps
        while right < len(hallway):
            if hallway[right] != '.':
                break
            right_steps += 1
            if right not in FORBIDDEN_POSITIONS.values():
                to_hallways.append([right_steps, right])
            right += 1
        
        for steps, pos in to_hallways:
            new_hallway = deepcopy(hallway)
            new_rooms = deepcopy(rooms)
            
            amphipod = new_rooms[room_num].pop(0)
            new_hallway[pos] = amphipod     
            
            next_moves.append((steps * POD_USED_ENERGIES[amphipod], (new_rooms, new_hallway)))

    return next_moves
    
def next_hall_moves(state):
    rooms, hallway = state

    next_moves = []
    for hall_pos, hall_amphipod in enumerate(hallway):
        if hall_amphipod == '.':
            continue

        room_num = "ABCD".index(hall_amphipod)
        room = rooms[room_num]
        
        if len(room) == 2:
            continue

        if len(room) > 0 and any(hall_amphipod != amphipod for amphipod in room):
            continue

        destination = FORBIDDEN_POSITIONS[hall_amphipod]
        dir = 1 if hall_pos < destination else -1
        energy_used = 0
        collision = False
        current_pos = hall_pos
        while current_pos != destination:
            current_pos += dir
            energy_used += POD_USED_ENERGIES[hall_amphipod]
            if hallway[current_pos] != '.':
                collision == True
                break
        
        if collision:
            continue
            
        new_hallway = deepcopy(hallway)
        new_hallway[hall_pos] = '.'
        
        new_rooms = deepcopy(rooms)
        new_rooms[room_num] = [hall_amphipod, *new_rooms[room_num]]
        
        energy_used += (2 - len(room)) * POD_USED_ENERGIES[hall_amphipod]
        
        next_moves.append((energy_used,(new_rooms, new_hallway)))
    # print(next_moves)
    return next_moves

def rooms_inorder(rooms):
    return rooms == [['A','A'], ['B','B'], ['C','C'], ['D','D']]

def solution(filename):
    state = parse_input(filename)
    heap = [(0, state)]
    energy_used = {}
    # cnt = 0
    while heap:
        energy, current_state = heapq.heappop(heap)
        # print(current_state[0])
        if rooms_inorder(current_state[0]):
            print(current_state)
            print(energy)
            break

        next_moves = next_room_moves(current_state) + next_hall_moves( current_state)

        for n_energy, n_state in next_moves:
            updated = energy + n_energy
            key = f"{n_state[0]}{n_state[1]}"
            if key not in energy_used or updated < energy_used[key]:
                energy_used[key] = updated
                heapq.heappush(heap, (updated, n_state))

solution("./inputs/q23.txt")