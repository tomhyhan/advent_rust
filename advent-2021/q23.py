import heapq
from copy import deepcopy
import json

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
    arrangements = [[char for char in line if 'A' <= char <= 'D']for line in lines[2:-1]]
    return (list(map(list, zip(*arrangements))), list(lines[1].strip('#')))

def next_room_moves(state, room_cnt):
    rooms, hallway = state

    next_moves = []
    for room_idx, room in enumerate(rooms):
        if not room:
            continue

        room_amphipods = 'ABCD'[room_idx]

        if all(pod == room_amphipods for pod in room):
            continue

        steps = (room_cnt - len(room)) + 1
        next_moves_inner = []

        # Move left
        left_steps = steps
        curr_pos = FORBIDDEN_POSITIONS[room_amphipods] - 1
        while curr_pos >= 0:
            if hallway[curr_pos] != '.':
                break
            left_steps += 1
            if curr_pos not in FORBIDDEN_POSITIONS.values():
                next_moves_inner.append([curr_pos, left_steps])
            curr_pos -= 1

        # Move right
        right_steps = steps
        curr_pos = FORBIDDEN_POSITIONS[room_amphipods] + 1
        while curr_pos < len(hallway):
            if hallway[curr_pos] != '.':
                break
            right_steps += 1
            if curr_pos not in FORBIDDEN_POSITIONS.values():
                next_moves_inner.append([curr_pos, right_steps])
            curr_pos += 1
        
        for hall_pos, steps in next_moves_inner:
            next_hallway = deepcopy(hallway)
            next_rooms = deepcopy(rooms)
            amphipod = next_rooms[room_idx].pop(0)
            next_hallway[hall_pos] = amphipod
            
            next_moves.append((steps * POD_USED_ENERGIES[amphipod], (next_rooms, next_hallway)))

    return next_moves
    
def next_hall_moves(state, room_cnt):
    rooms, hallway = state

    next_moves = []
    for hall_pos, amphipod in enumerate(hallway):
        if amphipod == '.':
            continue

        target_room = 'ABCD'.index(amphipod)
        room_occupancy = len(rooms[target_room])

        if room_occupancy == room_cnt:
            continue

        if room_occupancy > 0 and any(pod != amphipod for pod in rooms[target_room]):
            continue

        curr_pos = hall_pos

        step = 1 if FORBIDDEN_POSITIONS[amphipod] > hall_pos else -1
        energy_used = 0
        is_collision = False

        while curr_pos != FORBIDDEN_POSITIONS[amphipod]:
            curr_pos += step
            energy_used += POD_USED_ENERGIES[amphipod]
            if hallway[curr_pos] != '.':
                is_collision = True
                break
        if is_collision:
            continue

        energy_used += (room_cnt - room_occupancy) * \
            POD_USED_ENERGIES[amphipod]

        next_hallway = deepcopy(hallway)
        next_hallway[hall_pos] = '.'

        next_rooms = deepcopy(rooms)
        next_rooms[target_room] = [amphipod, *next_rooms[target_room]]
        
        next_moves.append((energy_used,(next_rooms, next_hallway)))
    # print(next_moves)
    return next_moves

def rooms_inorder(rooms, room_cnt):
    return rooms == [['A'] * room_cnt, ['B'] * room_cnt, ['C'] * room_cnt, ['D'] * room_cnt]

def solution(filename):
    state = parse_input(filename)
    heap = [(0, state)]
    energy_used = {}
    print(state)
    # cnt = 0
    room_cnt = len(state[0][0])
    print(room_cnt)
    while heap:
        energy, current_state = heapq.heappop(heap)
        # print(current_state[0])
        if rooms_inorder(current_state[0], room_cnt):
            print(current_state)
            print(energy)
            break

        next_moves =  next_hall_moves(current_state, room_cnt) + next_room_moves(current_state, room_cnt) 

        for n_energy, n_state in next_moves:
            updated = energy + n_energy
            # key = f"{n_state[0]}{n_state[1]}"
            key = json.dumps(n_state)
            if key not in energy_used or updated < energy_used[key]:
                energy_used[key] = updated
                heapq.heappush(heap, (updated, n_state))
        # break
solution("./inputs/q23.txt")