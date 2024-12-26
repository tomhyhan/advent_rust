

data = open("input/day24.txt").read()

OPS = {
    'XOR' : lambda a,b: a ^ b,
    'OR' : lambda a,b: a | b,
    'AND' : lambda a,b: a & b,
}

init_values, gate = data.split('\n\n')

info = {}
opp_info = {}

for line in init_values.splitlines():
    s, d = line.split(':', 1)
    info[s] = int(d)
    
for line in gate.splitlines():
    d, s = line.split(" -> ")
    info[s] = tuple(d.split(' '))
    opp_info[tuple(d.split(' '))] = s
    
def getz(key):
    value = info[key]
    if isinstance(value, int):
        return value
    f, op, s = value
    return OPS[op](getz(f), getz(s))

total = 0 
for key in info:
    if key.startswith('z'):
        pos = int(key[1:])
        zval = getz(key)
        total |= (zval << pos)

print("Part1: ", total)

# part2

def get_total(char, info):
    total = 0 
    for key in info:
        if key.startswith(char):
            pos = int(key[1:])
            total |= (info[key] << pos)
    return total
    
def getxsum(info):
    return get_total('x', info)

def getysum(info):
    return get_total('y', info)

def getzsum(info):
    total = 0 
    for key in info:
        if key.startswith('z'):
            pos = int(key[1:])
            zval = getz(key)
            total |= (zval << pos)
    return total

def addz(key, debug):
    value = info[key]
    if isinstance(value, int):
        return value
    f, op, s = value
    debug.add(key)
    return OPS[op](addz(f, debug), addz(s,debug))

def debugz(char):
    debug = set()
    addz(char, debug)
    return debug
            
from copy import deepcopy

def get_gate(src, dest, op, diagram):
    f = (src, op, dest)
    s = (dest, op, src)
    try:
        return  diagram[f] if f in diagram else diagram[s]
    except:
        return ()
def buildz():
    zs = [{"z00"}, {"z01", "wwc", "nvv"}]
    prev_z = []
    for i in range(2, 50):
        pz = f"z0{i-1}" if i < 11 else f"z{i-1}" 
        z = f"z0{i}" if i < 10 else f"z{i}"
        px = f"x0{i-1}" if i < 11 else f"x{i-1}" 
        x = f"x0{i}" if i < 10 else f"x{i}"
        py = f"y0{i-1}" if i < 11 else f"y{i-1}" 
        y = f"y0{i}" if i < 10 else f"y{i}"
        if z not in info:
            continue
        prev_z = deepcopy(zs[-1])
        prev_z.remove(pz)
        new_z = prev_z
        
        paddr = info[pz]
        paddr_key = get_gate(paddr[0], paddr[2], "AND", opp_info)
        new_z.add(paddr_key)
        
        prev_and_key = get_gate(px, py, "AND", opp_info)
        new_z.add(prev_and_key)
        
        can_add_key = get_gate(paddr_key, prev_and_key, "OR", opp_info)
        new_z.add(can_add_key)
        
        curr_add = get_gate(x, y, "XOR", opp_info)
        new_z.add(curr_add)
        
        new_z.add(z)
        if tuple() in new_z:
            new_z.remove(tuple())
        zs.append(new_z)
    return zs
        

# qjb gvw
def change_info(src, des):
    info[src],info[des] = info[des],info[src]

# manual search ;p
change_info("qjb", "gvw")
change_info("jgc", "z15")
change_info("drg", "z22")
change_info("jbp", "z35")

opp_info = {v:k for k, v in info.items() if not isinstance(v, int)}
zs = buildz()

for i in range(50):
    z = f"z0{i}" if i < 10 else f"z{i}" 
    if z not in info:
        break
    d = debugz(z)
    if zs[i] != d:
        print(i)
        print(zs[i]) 
        print(d) 
        print("diff: ", zs[i] - d)
        print("diff: ", d - zs[i])
        break       
    
print("Part2: ", ','.join(sorted(["qjb", "gvw", "jgc", "z15", "drg", "z22", "jbp", "z35"])))         
