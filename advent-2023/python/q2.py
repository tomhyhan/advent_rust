from functools import reduce
from operator import mul

BAGS = {
    "red": 12,
    "green": 13,
    "blue": 14
}
MIN = float("-inf")


def is_valid_round(round):
    cards = round.split(',')
    for card in cards:
        num, color = card.strip().split(' ')
        if BAGS[color] < int(num):
            return False
    return True


def part1(lines):
    result = 0
    for idx, line in enumerate(lines):
        game_id = idx + 1
        game = line.split(':')
        rounds = game[1].split(';')
        is_valid = True
        for r in rounds:
            if not is_valid_round(r):
                is_valid = False
        if is_valid:
            print(game_id)
            result += game_id
    print(result)


def part2(lines):
    result = 0
    for line in lines:
        game = line.split(':')
        rounds = game[1].split(';')
        min_cards = {
            "red": MIN,
            "green": MIN,
            "blue": MIN
        }
        for r in rounds:
            cards = r.split(',')
            for card in cards:
                num, color = card.strip().split(' ')
                min_cards[color] = max(min_cards[color], int(num))
        result += reduce(mul, min_cards.values())
    print(result)


if __name__ == "__main__":
    filename = "./inputs/q2.txt"
    lines = open(filename).read().split('\n')
    # part1(lines)
    part2(lines)
    # merge them together
