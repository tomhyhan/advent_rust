from functools import reduce


def part1(lines):
    return sum([int((1 << len(reduce(lambda x, y: x & y, map(lambda x: set(x.split()), line.split(':')[1].split('|'))))) / 2)
                for line in lines])


def part2(lines):
    matchings = [len(reduce(lambda x, y: x & y, map(lambda x: set(x.split()), line.split(':')[1].split('|'))))
                 for line in lines]
    scratchcards = [1] * len(matchings)
    for card, matching in enumerate(matchings):
        for card_copy in range(card+1, card+matching+1):
            scratchcards[card_copy] += scratchcards[card]
    print(sum(scratchcards))


def solution(file):
    lines = open(file).read().split('\n')
    print(part1(lines))
    part2(lines)


solution("./inputs/q4.txt")
