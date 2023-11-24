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
    return max(abs(diff.x) + abs(diff.y) + abs(diff.z))

def fingerprint(point1, point2):
    return (manhattan_distance(point1, point2) + max_coord_distance(point1, point2))

def find_match(known_beacons, known_fingerprints, report, report_fingerprints):
    pass


def map_beacons(reports):
    current_beacons = reports.pop(0)
    fingerprints = defaultdict()
    for p1, p2 in combinations(current_beacons, 2):
        fingerprints[fingerprint(p1,p2)].append((p1,p2))  
    
    yield (Point3D(0,0,0), current_beacons)
    
    possible_fingerprints = []
    for report in reports:
        pass
    
    pass


def part1():
    input = open("./inputs/q19.txt").read()
    scanners = parse_scanner_reports(input)
    print(scanners)
    _ , beacons = map_beacons(scanners)

def parse_scanner_reports(input):
    return [[Point3D(*map(int, beacon.split(','))) for beacon in scanners.split('\n') if beacon[1] != '-'] for scanners in input.split("\n\n") ]

# solution()
part1()