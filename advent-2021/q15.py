file = """Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
    row = 10
    sensors = []
    for line in lines:
        tokens = line.split()
        x = parse(tokens[2])
        y = parse(tokens[3])
        bx = parse(tokens[-2])
        by = parse(tokens[-1])
        sensors.append(Sensor(x,y,md(x,y,bx,by)))

    beacon_ranges_overlap = []
    for sensor in sensors:
        min_x = sensor.range - abs(sensor.y - row) - sensor.x  
        max_x = sensor.range - abs(sensor.y - row) + sensor.x  
        print(sensor)
        print(min_x, max_x)
        print()
        beacon_ranges_overlap.append([min_x, max_x])

    # beacon_ranges_overlap.sort(key=lambda x: (x[0], -x[1]))
    # for range in beacon_ranges_overlap:
        
    
    print(beacon_ranges_overlap)
solution("filename")