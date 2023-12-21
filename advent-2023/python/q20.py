from collections import deque, defaultdict
# % (ff) -> default: off
#    -> high: nothing
#    -> low: if OFF, turns ON sends high
#            if ON, turns OFF sends low

# & (con): connected modules' type
#       -> default: low pulse
#       -> if all highs, sends low
#       -> else, sends high

class FlipFlob:
    def __init__(self, src, dest):
        self.on = False
        self.src = src
        self.dest = dest
        self.sendout = "low"

    
    def click(self, from_, pulse):
        if pulse == "high":
            return False
        
        self.on = not self.on      
        self.sendout = "high" if self.on else "low"
        return True

    def __repr__(self):
        return f"FLIP {self.src} |"

class Conjunction:
    def __init__(self, src, dest):
        self.src = src
        self.dest = dest
        self.sendout = "low"
        self.conjunctions = {}

    def click(self, from_, pulse):
        self.conjunctions[from_] = pulse
        self.sendout = "low" if all(state == "high" for state in self.conjunctions.values()) else "high"
        return True

    def __repr__(self):
        return f"CON {self.src} {self.conjunctions}"

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
    
    for key, module in circuits.items():
        if key == "broadcaster":
            continue
        try:
            for con in module.dest:
                if isinstance(circuits[con], Conjunction):
                    circuits[con].conjunctions[key] = "low"
        except KeyError:
            pass

    return circuits

def button_press(start, end, circuits, idx):
    queue = deque([(start, end)])
    low, high = 0,0
    
    while queue:
        start, end = queue.popleft()
        if end == "broadcaster" or start == "broadcaster":
            sendout = "low"
        else:
            sendout = circuits[start].sendout

        low += sendout == "low"
        high += sendout == "high"        
        dests = circuits[end].dest if end != "broadcaster" else circuits[end]

        if end != "broadcaster":
            if not circuits[end].click(start, sendout):
                continue
        
        if end == "dg":
            for key in circuits["dg"].conjunctions:
                if  circuits["dg"].conjunctions[key] == "high":
                    print(key, idx)
            # return True
        
        for dest in dests:
            if dest not in circuits:
                # if circuits[end].sendout == "low":
                #     print("?")
                low += circuits[end].sendout == "low"
                high += circuits[end].sendout == "high"        
                continue 
            queue.append((end,dest))

    # return low, high
    return False
# {"lk": "low", "zv": "low", "sp": "low", "xt": "low"}
# jc - 15, vv - 3, xq - 1, dv - 7
def part1(circuits):
    # low, high = 0 ,0 
    for idx in range(10000):
        # blow, bhigh = button_press("button", "broadcaster", circuits)
        button_press("button", "broadcaster", circuits, idx)
        # if cycle:
        #     print(idx)
        # low += blow
        # high += bhigh
        # if circuits["dg"].conjunctions["lk"] == "high":
        #     print(idx)
        # print(circuits["dg"].conjunctions)
        # if circuits["jc"].sendout == "high":
        #     print(idx)
        # key = json.dumps(circuits["jc"].conjunctions)
        # key = json.dumps(circuits["jc"].conjunctions)
        # if all(state == "low" for state in circuits["dv"].conjunctions.values()):
        #     print(circuits["xq"].conjunctions)
        #     print(idx)
        #     break
        # visited.add(key)
    # 3822
    print(7533 - 3766)
    print(7645 - 3822)
    print(7857 - 3928)
    print(8101 - 4050)
    print(lcm(3767, 3823, 3929, 4051))
    print(lcm(2, 10))
    # 229215609826339
    # print(low * high)
# "low" if all(state == "high" for state in self.conjunctions.values()) else "high"

def gcd(x,y):
    return y if x % y == 0 else gcd(y, x%y)

def lcm(*args):
    total = args[0]
    for i in range(1, len(args)):
        total = (total * args[i]) // gcd(total, args[i])
    return total

def solution():
    filename = "./inputs/q20.txt"
    circuits= parse(filename)
    part1(circuits)
    pass


solution()

# def button_press(start, end, circuits):
#     queue = deque([(start, end)])
#     low, high = 0,0
    
#     while queue:
#         start, end = queue.popleft()
#         if end == "broadcaster" or start == "broadcaster":
#             sendout = "low"
#         else:
#             sendout = circuits[start].sendout

#         low += sendout == "low"
#         high += sendout == "high"        
#         dests = circuits[end].dest if end != "broadcaster" else circuits[end]

#         if end != "broadcaster":
#             if not circuits[end].click(start, sendout):
#                 continue
        
#         for dest in dests:
#             if dest not in circuits:
#                 # if circuits[end].sendout == "low":
#                 #     print("?")
#                 low += circuits[end].sendout == "low"
#                 high += circuits[end].sendout == "high"        
#                 continue 
#             queue.append((end,dest))

#     return low, high