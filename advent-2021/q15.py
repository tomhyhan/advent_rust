class Sensor:
    def __init__(self, x, y, range):
        self.x = x
        self.y = y
        self.range = range

    def __repr__(self):
        return f"{self.x} {self.y} {self.range}"
    
def parse(token):
    return int(token.split("=")[1].strip(",").strip(":"))

def parse_input(filename):
    lines = open(filename).read().split('\n')
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
    return sensors, beacons

def md(x,y,x1,y1):
    return abs(x-x1) + abs(y-y1)


def find_beacons_ranges(sensors, row):
    beacon_ranges_overlap = []
    for sensor in sensors:
        if sensor.range - abs(sensor.y - row) < 0:
            continue
        min_x = -(sensor.range - abs(sensor.y - row)) + sensor.x   
        max_x = sensor.range - abs(sensor.y - row)  + sensor.x
        beacon_ranges_overlap.append([min_x, max_x])

    beacon_ranges_overlap.sort(key=lambda x: (x[0], -x[1]))
    return beacon_ranges_overlap

def merge_beacons_ranges(beacon_ranges_overlap):
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
    return result

def part1(sensors, beacons, row):
    beacon_ranges_overlap = find_beacons_ranges(sensors, row)
    merge_results = merge_beacons_ranges(beacon_ranges_overlap)
    print(sum([result[1] - result[0] + 1 for result in merge_results]) - len([beacon for beacon in beacons if beacon[1] == row]))
    
def part2(sensors, rows):
    for i in range(rows):
        beacon_ranges_overlap = find_beacons_ranges(sensors, i)
        merge_results = merge_beacons_ranges(beacon_ranges_overlap)
        if len(merge_results) > 1:
            print(merge_results, i)
            print((merge_results[0][1] + 1) * 4000000 + i)
            break

def solution(filename):
    sensors, beacons = parse_input(filename) 
    part1(sensors, beacons, 2000000) 
    part2(sensors, 4000000)

solution("./q15.txt")