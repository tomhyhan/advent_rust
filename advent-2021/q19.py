from collections import deque
from itertools import product


orientations = [
    lambda x, y, z: (x, y, z),
    lambda x, y, z: (x, z, -y),
    lambda x, y, z: (x, -y, -z),
    lambda x, y, z: (x, -z, y),
    lambda x, y, z: (y, x, -z),
    lambda x, y, z: (y, z, x),
    lambda x, y, z: (y, -x, z),
    lambda x, y, z: (y, -z, -x),
    lambda x, y, z: (z, x, y),
    lambda x, y, z: (z, y, -x),
    lambda x, y, z: (z, -x, -y),
    lambda x, y, z: (z, -y, x),
    lambda x, y, z: (-x, y, -z),
    lambda x, y, z: (-x, z, y),
    lambda x, y, z: (-x, -y, z),
    lambda x, y, z: (-x, -z, -y),
    lambda x, y, z: (-y, x, z),
    lambda x, y, z: (-y, z, -x),
    lambda x, y, z: (-y, -x, -z),
    lambda x, y, z: (-y, -z, x),
    lambda x, y, z: (-z, x, -y),
    lambda x, y, z: (-z, y, x),
    lambda x, y, z: (-z, -x, y),
    lambda x, y, z: (-z, -y, -x),
]

def map_beacons(beacons, other_beacons):

    for orientation in orientations:
        relative_pos = [orientation(*pos) for pos in other_beacons]

        distances = set([(x-x1, y-y1, z-z1) for (x,y,z), (x1,y1,z1) in product(beacons, relative_pos)])
        
        for dx,dy,dz in distances:
            possible_beacons = set([(dx+x, dy+y, dz+z)  for x,y,z in relative_pos])
            if len(beacons & possible_beacons) >= 12:
                beacons |= possible_beacons
                return (dx,dy,dz)
    return False

def solution():
    input = open("./inputs/q19.txt").read()
    scanners = deque(parse_scanners(input))

    beacons = scanners.popleft()
    distances = []
    while scanners:
        other_beacons = scanners.popleft()
        if not (distance := map_beacons(beacons, other_beacons)):
            scanners.append(other_beacons)
        if distance:    
            distances.append(distance)
    
    maxd = 0
    for i in range(len(distances)):
        for j in range(i+1, len(distances)):
            x,y,z = distances[i]
            x1,y1,z1 = distances[j]
            maxd = max(maxd, abs(x-x1) + abs(y-y1) + abs(z-z1))
            
    print(maxd)

def parse_scanners(input):
    return [set(tuple(map(int, beacon.split(',')))
                for beacon in scanner.split('\n') if beacon[1] != '-')
            for scanner in input.split('\n\n')]

solution()

# mahattan((3,3), (4,1))
# mahattan((3,3), (0,2))
# mahattan((-2,1), (-1,-1))
# mahattan((-2,1), (-5,0))