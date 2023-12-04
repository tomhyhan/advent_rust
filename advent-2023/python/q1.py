import re


def part1(file):
    nums = [[ch for ch in line if ch.isdigit()]
            for line in open(file).read().split('\n')]
    total = sum([int(num[0] + num[-1]) for num in nums])
    print(total)


num_dt = {"one": 1, "two": 2, "three": 3, "four": 4,
          "five": 5, "six": 6, "seven": 7, "eight": 8, "nine": 9}
"three eight"
"eight three"
"five eight"
""


def gen_overlaps():
    overlaps = []
    for num_s in num_dt.keys():
        for other_num in num_dt.keys():
            if num_s[-1] == other_num[0]:
                overlaps.append((num_s + other_num, num_s + other_num[1:]))
    return overlaps


def replace_line(line, overlaps):
    for overlap in overlaps:
        des, src = overlap
        line = line.replace(src, des)
    return line


def part2(file):
    pattern = r"one|two|three|four|five|six|seven|eight|nine|[1-9]"
    overlaps = gen_overlaps()
    searches = [re.findall(pattern, replace_line(line, overlaps))
                for line in open(file).read().split('\n')]
    total = 0
    for s in searches:
        # print(s)
        first = num_dt[s[0]] if not s[0].isdigit() else s[0]
        last = num_dt[s[-1]] if not s[-1].isdigit() else s[-1]
        total += int(str(first) + str(last))
    print(total)


if __name__ == "__main__":
    file = "./inputs/q1.txt"
    # part1(file)
    part2(file)
    # use start with
