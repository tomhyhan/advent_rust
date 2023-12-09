def part1(reports):
    result = 0
    result_p2 = 0
    for report in reports:
        current = report
        sequences = [current]
        while any(current):
            new_s = [current[i] - current[i-1] for i in range(1, len(current))]
            sequences.append(new_s)
            current = new_s
        next_history = 0
        prev_history = 0
        for i in reversed(range(len(sequences) -1)):
            next_history += sequences[i][-1]
            prev_history = sequences[i][0] - prev_history
        result += next_history
        result_p2 += prev_history
        print(prev_history)
    print(result)
    print(result_p2)
    
def solution():
    filename = "./inputs/q9.txt"
    reports = [list(map(int, line.split())) for line in open(filename).read().split('\n')]
    # improve with recursion
    part1(reports)
    pass

solution()