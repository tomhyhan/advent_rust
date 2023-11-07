class Universes:
    def __init__(self, universe1, universe2):
        self.universe1 = universe1
        self.universe2 = universe2

    def __repr__(self):
        return f"{self.universe1} {self.universe2}"

class Player:
    def __init__(self, pos, value):
        self.pos = pos
        self.value = value

class Players:
    def __init__(self, p1, p2):
        self.p1 = p1
        self.p2 = p2

    def __eq__(self, other):
        return (
            self.p1.pos == other.p1.pos
            and self.p2.pos == other.p2.pos
            and self.p1.value == other.p1.value
            and self.p2.value == other.p2.value
        )

    def __hash__(self):
        return hash((self.p1.pos, self.p2.pos, self.p1.value, self.p2.value))


def play_universe(players, memo, p1_turn):
    if players.p1.value >= 21:
        return Universes(1,0)

    if players.p2.value >= 21:
        return Universes(0,1)

    key = (players.p1.pos, players.p2.pos, players.p1.value, players.p2.value, p1_turn)
    if key in memo:
        return memo[key]
    
    universes = Universes(0,0)
    
    for d1 in [1,2,3]:
        for d2 in [1,2,3]:
            for d3 in [1,2,3]:
                new_p1 = Player(players.p1.pos, players.p1.value)
                new_p1.pos = (new_p1.pos + d1 + d2 + d3) % 10
                new_p1.value += new_p1.pos + 1

                new_p2 = Player(players.p2.pos, players.p2.value)

                new_players = Players(new_p2,new_p1)
                
                other_universe = play_universe(new_players, memo, False)
                universes.universe1 += other_universe.universe2
                universes.universe2 += other_universe.universe1
                # if p1_turn:
                #     new_p1 = Player(players.p1.pos, players.p1.value)
                #     new_p1.pos = (new_p1.pos + d1 + d2 + d3) % 10
                #     new_p1.value += new_p1.pos + 1

                #     new_p2 = Player(players.p2.pos, players.p2.value)

                #     new_players = Players(new_p1,new_p2)
                    
                #     other_universe = play_universe(new_players, memo, False)
                #     universes.universe1 += other_universe.universe1
                #     universes.universe2 += other_universe.universe2
                # else:
                #     new_p1 = Player(players.p1.pos, players.p1.value)
                    
                #     new_p2 = Player(players.p2.pos, players.p2.value)
                #     new_p2.pos = (new_p2.pos + d1 + d2 + d3) % 10
                #     new_p2.value += new_p2.pos + 1

                #     new_players = Players(new_p1,new_p2)
                    
                #     other_universe = play_universe(new_players, memo, True)
                #     universes.universe1 += other_universe.universe1
                #     universes.universe2 += other_universe.universe2

    memo[key] = universes
    return universes

def solution():
    p1 = Player(5-1,0)
    p2 = Player(10-1,0)
    memo = {}
    players = Players(p1, p2)
    
    universe = play_universe(players, memo, True)
    print(universe)
    
solution()