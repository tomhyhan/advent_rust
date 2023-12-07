# important !
import re 

def part1(races):
    # brute force
    result = 1
    for time, distance in races:
        possible_wins = 0
        for i in range(1, time):
            speed = i
            left = time - speed
            if speed * left > distance:
                possible_wins += 1
        result *= possible_wins
    print(result)
# 49979494

def part2(time , distance):
    left, right = 1, time - 1
    while left + 1 < right:
        speed = round((left + right) / 2)
        time_left = time - speed
        if speed * time_left >= distance:
            right = speed
        else:
            left = speed 
    lowerbound = right
    highbound = (time/2) - (lowerbound - (time/2))
    print(highbound - lowerbound + 1)


def parse_line(line):
    return map(int, re.findall(r"\d+", line))

def parse_line2(line):
    return re.findall(r"\d+", line)

def solution():
    file = "./inputs/q6.txt"
    lines = open(file).read().split('\n')
    times = parse_line(lines[0])
    distances = parse_line(lines[1])
    # part1(list(zip(times, distances)))
    part2(int(''.join(parse_line2(lines[0]))), int(''.join(parse_line2(lines[1]))))
    # improve with symmetry
    pass

solution()