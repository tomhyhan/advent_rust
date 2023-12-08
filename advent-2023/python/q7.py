from enum import IntEnum
from collections import Counter

class Types(IntEnum):
    FIVE_KIND = 1
    FOUR_KIND = 2
    FULL_HOUSE = 3
    THREE_KIND = 4
    TWO_PAIR = 5
    ONE_PAIR = 6
    HIGH_CARD = 7

# CARD_RANKS = {
#     "A": 'e', 
#     "K": 'd', 
#     "Q": 'c', 
#     "J": 'b',
#     "T": 'a',
# }
# part2
CARD_RANKS = {
    "A": 'e', 
    "K": 'd', 
    "Q": 'c', 
    "J": '1',
    "T": 'a',
}

class Pair:
    def __init__(self, hand, bid, ctype):
        self.hand = hand
        self.bid = bid
        self.ctype = ctype
    
    def __repr__(self):
        return f"{self.hand} {self.ctype}"
    
    def __lt__(self, other):
        if self.ctype == other.ctype:
            for a,b in zip(self.hand, other.hand):
                a =  CARD_RANKS[a] if a in CARD_RANKS else a
                b =  CARD_RANKS[b] if b in CARD_RANKS else b
                if a != b:
                    return a < b
            return True
        else:
            return self.ctype > other.ctype

def part1(pairs):
    pairs.sort()
    print(pairs)
    winnings = sum([pair.bid * (idx+1) for idx, pair in enumerate(pairs)])
    # 249467918 X
    # 249620439 X
    # 250254244
    # 250606082 X
    print(winnings)

def parse_type(hand):
    counter = Counter(hand)
    max_cards = [[cnt, card] for card, cnt in counter.items() if card != 'J'] 
    if max_cards:
        max_card = max(max_cards)
        hand = hand.replace('J', max_card[1])
    counter = Counter(hand)
    c_len = len(counter)
    if c_len == 1:
        return Types.FIVE_KIND
    elif c_len == 2:
        if 4 in counter.values():
            return Types.FOUR_KIND
        else:
            return Types.FULL_HOUSE
    elif c_len == 3:
        if 3 in counter.values():
            return Types.THREE_KIND
        else:
            return Types.TWO_PAIR
    elif c_len == 4:
        return Types.ONE_PAIR
    elif c_len == 5:
        return Types.HIGH_CARD

def parse(lines):
    pairs = []
    for line in lines:
        hand, bid = line.split()
        ctype = parse_type(hand)
        p = Pair(hand, int(bid), ctype)
        pairs.append(p)
    return pairs

def solution():
    filename = "./inputs/q7.txt"
    lines = open(filename).read().split('\n')
    pairs = parse(lines)
    part1(pairs)    

#improve with entropy
# def eval(line):
#     hand, bid = line.split()
#     hand = hand.translate(str.maketrans('TJQKA', face))
#     print(hand)
#     best = max(type(hand.replace('0', r)) for r in hand)
#     return best, hand, int(bid)

# def type(hand):
#     return sorted(map(hand.count, hand), reverse=True)

# for face in 'ABCDE', 'A0CDE':
#     print(sum(rank * bid for rank, (*_, bid) in
#         enumerate(sorted(map(eval, open('./inputs/q7.txt'))), start=1)))
    
# solution()