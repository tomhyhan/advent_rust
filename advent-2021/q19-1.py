from dataclasses import dataclass

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

def map_beacons(reports):
    first_report = reports.pop()
    print(first_report)

def solution():
    input = open("./inputs/q19.txt").read()
    reports = parse_scanner_reports(input)
    map_beacons(reports)
    # for _, beacons in map_beacons(reports):
    #     for beacon in beacons:
    #         pass
    
def parse_scanner_reports(input):
    return [[Point3D(*map(int, beacon.split(',')))
             for beacon in scanner.split('\n') if beacon[1] != '-']
            for scanner in input.split('\n\n')]

solution()