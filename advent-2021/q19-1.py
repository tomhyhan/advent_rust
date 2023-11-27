from dataclasses import dataclass
from itertools import combinations
from collections import defaultdict


@dataclass(frozen=True)
class Point3D:
    x: int
    y: int
    z: int

    def __add__(self, other):
        if not isinstance(other, Point3D):
            return NotImplemented
        return Point3D(self.x + other.x, self.y + other.y, self.z + other.z)

    def __mul__(self, other):
        if not isinstance(other, Point3D):
            return NotImplemented
        return Point3D(self.x * other.x, self.y * other.y, self.z * other.z)

    def __sub__(self, other):
        if not isinstance(other, Point3D):
            return NotImplemented
        return Point3D(self.x - other.x, self.y - other.y, self.z - other.z)


# http://www.markronan.com/mathematics/symmetry-corner/the-rotations-of-a-cube/
rotations = [
    lambda p: Point3D(p.x, p.y, p.z),
    lambda p: Point3D(p.x, p.z, -p.y),
    lambda p: Point3D(p.x, -p.y, -p.z),
    lambda p: Point3D(p.x, -p.z, p.y),
    lambda p: Point3D(p.y, p.x, -p.z),
    lambda p: Point3D(p.y, p.z, p.x),
    lambda p: Point3D(p.y, -p.x, p.z),
    lambda p: Point3D(p.y, -p.z, -p.x),
    lambda p: Point3D(p.z, p.x, p.y),
    lambda p: Point3D(p.z, p.y, -p.x),
    lambda p: Point3D(p.z, -p.x, -p.y),
    lambda p: Point3D(p.z, -p.y, p.x),
    lambda p: Point3D(-p.x, p.y, -p.z),
    lambda p: Point3D(-p.x, p.z, p.y),
    lambda p: Point3D(-p.x, -p.y, p.z),
    lambda p: Point3D(-p.x, -p.z, -p.y),
    lambda p: Point3D(-p.y, p.x, p.z),
    lambda p: Point3D(-p.y, p.z, -p.x),
    lambda p: Point3D(-p.y, -p.x, -p.z),
    lambda p: Point3D(-p.y, -p.z, p.x),
    lambda p: Point3D(-p.z, p.x, -p.y),
    lambda p: Point3D(-p.z, p.y, p.x),
    lambda p: Point3D(-p.z, -p.x, p.y),
    lambda p: Point3D(-p.z, -p.y, -p.x),
]


def max_coord_distance(point1, point2):
    diff = point1 - point2
    return max(abs(diff.x), abs(diff.y), abs(diff.z))

def manhattan_distance(point1, point2):
    diff = point1 - point2
    return abs(diff.x) + abs(diff.y) + abs(diff.z)

def fingerprint(point1, point2):
    return (manhattan_distance(point1, point2) + max_coord_distance(point1, point2))

def find_match(known_beacons, known_fingerprints, report, report_fingerprints):
    matching_fgp = [(fgp, report_pair) for (fgp, report_pair) in report_fingerprints if fgp in known_fingerprints]

    for fgp, report_pair in matching_fgp:
        for pair in known_fingerprints[fgp]:
            p1, p2 = pair
            rp1, rp2 = report_pair
            
            for rotation in rotations:
                offset = p1 - rotation(rp1)
                if offset != p2 - rotation(rp2):
                    continue
                transforms = [rotation(p) + offset for p in report]
                relatives = 0
                for transform in transforms:
                    if transform in known_beacons:
                        relatives += 1
                    if relatives >= 12:
                        return (offset, transforms)


def map_beacons(reports):
    current_beacons = set(reports.pop(0))
    fingerprints = defaultdict(list)
    for p1, p2 in combinations(current_beacons, 2):
        fingerprints[fingerprint(p1,p2)].append((p1,p2))  
    
    yield (Point3D(0,0,0), current_beacons)
    
    possible_fingerprints = []
    for report in reports:
        fgs = [(fingerprint(p1,p2), (p1,p2)) for p1, p2 in combinations(report, 2)]  
        possible_fingerprints.append([report, fgs])

    while possible_fingerprints:
        for idx, (report, fgs) in enumerate(possible_fingerprints):
            if (result := find_match(current_beacons, fingerprints, report ,fgs)):
                yield result                
                _, transformed_report = result
                for p1, p2 in combinations(transformed_report,2):
                    fingerprints[fingerprint(p1,p2)].append((p1, p2))
                current_beacons.update(transformed_report)
                del possible_fingerprints[idx]
                break
            
def part1():
    input = open("./inputs/q19.txt").read()
    scanners = parse_scanner_reports(input)
    beacons_map = set()
    for (_, beacons) in map_beacons(scanners):
        for beacon in beacons:
            beacons_map.add(beacon)
    print(len(beacons_map))
    
    # x = list(p for p in combinations([(0,2), (4,1), (3,3)], 2))
    # y = list(p for p in combinations([(-1,-1), (-5,0), (-2,1)], 2))
    # for p1, p2 in x:
    #     for r1, r2 in y:
    #         print(p1,p2,r1,r2)
    #         print(p1[0] - r1[0], p2[1] - r2[1])
    #         print()
def parse_scanner_reports(input):
    return [[Point3D(*map(int, beacon.split(','))) for beacon in scanners.split('\n') if beacon[1] != '-'] for scanners in input.split("\n\n") ]

# solution()
part1()