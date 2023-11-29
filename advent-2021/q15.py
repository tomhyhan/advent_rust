file = """Sensor at x=3088287, y=2966967: closest beacon is at x=3340990, y=2451747
Sensor at x=289570, y=339999: closest beacon is at x=20077, y=1235084
Sensor at x=1940197, y=3386754: closest beacon is at x=2010485, y=3291030
Sensor at x=1979355, y=2150711: closest beacon is at x=1690952, y=2000000
Sensor at x=2859415, y=1555438: closest beacon is at x=3340990, y=2451747
Sensor at x=1015582, y=2054755: closest beacon is at x=1690952, y=2000000
Sensor at x=1794782, y=3963737: closest beacon is at x=2183727, y=4148084
Sensor at x=2357608, y=2559811: closest beacon is at x=2010485, y=3291030
Sensor at x=2936, y=1218210: closest beacon is at x=20077, y=1235084
Sensor at x=2404143, y=3161036: closest beacon is at x=2010485, y=3291030
Sensor at x=12522, y=1706324: closest beacon is at x=20077, y=1235084
Sensor at x=1989162, y=3317864: closest beacon is at x=2010485, y=3291030
Sensor at x=167388, y=3570975: closest beacon is at x=-1018858, y=4296788
Sensor at x=1586527, y=2233885: closest beacon is at x=1690952, y=2000000
Sensor at x=746571, y=1442967: closest beacon is at x=20077, y=1235084
Sensor at x=3969726, y=3857699: closest beacon is at x=3207147, y=4217920
Sensor at x=1403393, y=2413121: closest beacon is at x=1690952, y=2000000
Sensor at x=2343717, y=3649198: closest beacon is at x=2183727, y=4148084
Sensor at x=1473424, y=688269: closest beacon is at x=2053598, y=-169389
Sensor at x=2669347, y=190833: closest beacon is at x=2053598, y=-169389
Sensor at x=2973167, y=3783783: closest beacon is at x=3207147, y=4217920
Sensor at x=2011835, y=3314181: closest beacon is at x=2010485, y=3291030
Sensor at x=1602224, y=2989728: closest beacon is at x=2010485, y=3291030
Sensor at x=3928889, y=1064434: closest beacon is at x=3340990, y=2451747
Sensor at x=2018358, y=3301778: closest beacon is at x=2010485, y=3291030
Sensor at x=1811905, y=2084187: closest beacon is at x=1690952, y=2000000
Sensor at x=1767697, y=1873118: closest beacon is at x=1690952, y=2000000
Sensor at x=260786, y=1154525: closest beacon is at x=20077, y=1235084"""



"""Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"""
class Sensor:
    def __init__(self, x, y, range):
        self.x = x
        self.y = y
        self.range = range


    def __repr__(self):
        return f"{self.x} {self.y} {self.range}"
    
def parse(token):
    return int(token.split("=")[1].strip(",").strip(":"))

def md(x,y,x1,y1):
    return abs(x-x1) + abs(y-y1)

def solution(filename):
    lines = file.splitlines()
    sensors = []
    beacons = set()
    for line in lines:
        tokens = line.split()
        x = parse(tokens[2])
        y = parse(tokens[3])
        bx = parse(tokens[-2])
        by = parse(tokens[-1])
        sensors.append(Sensor(x,y,md(x,y,bx,by)))
        beacons.add((bx,by))

    for i in range(4000000):
        row = i
        beacon_ranges_overlap = []
        for sensor in sensors:
            if sensor.range - abs(sensor.y - row) < 0:
                continue
            min_x = -(sensor.range - abs(sensor.y - row)) + sensor.x   
            max_x = sensor.range - abs(sensor.y - row)  + sensor.x
            beacon_ranges_overlap.append([min_x, max_x])

        beacon_ranges_overlap.sort(key=lambda x: (x[0], -x[1]))

        result = []
        beacon = beacon_ranges_overlap[0]
        for idx in range(1, len(beacon_ranges_overlap)):
            start, end = beacon_ranges_overlap[idx]
            if start <= beacon[1]:
                beacon[1] = end if end > beacon[1] else beacon[1]
            else:
                result.append(beacon)
                beacon = beacon_ranges_overlap[idx]
        result.append(beacon)
        if len(result) > 1:
            print(result, i)
            break
    print(2721114 * 4000000 + 3367718)
    
    # result = beacon[1] + abs(beacon[0]) + 1

    # for bx,by in beacons:
    #     if by == row and beacon[0] <= bx <= beacon[1]:
    #         result -= 1      
    # print(result)

solution("filename")