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

CARD_RANKS = {
    "A": 'e', 
    "K": 'd', 
    "Q": 'c', 
    "J": 'b',
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
    print(winnings)

def parse_type(hand):
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

solution()