import re
from copy import deepcopy
from functools import reduce
from operator import mul

MIN = 1
MAX = 4000

CMP_FUNCTIONS = {
    '<': lambda x,y : x < y,
    '>': lambda x,y : x > y,
}

class Rule:
    def __init__(self, src, cmp, num, des):
        self.src = src
        self.cmp = cmp
        self.num = int(num)
        self.des = des

    def get_num(self):
        #  <= < max
        # >= > min
        pass
    
    def match(self,part):
        src_num = part[self.src]
        return CMP_FUNCTIONS[self.cmp](src_num, self.num)

    def create_opp(self):
        cmp =  "<=" if self.cmp == '>' else '>=' 
        return Rule(self.src, cmp, self.num, self.des)

    def cmp_range(self, valid_ranges):
        if self.cmp == "<=" or self.cmp == '<':
            offset = 0 if self.cmp == "<=" else 1 
            # print("valid_ranges", valid_ranges)
            valid_ranges[self.src][1] = min(valid_ranges[self.src][1], self.num - offset)
        elif self.cmp in ">=" or self.cmp == '>':
            offset = 0 if self.cmp == ">=" else 1 
            valid_ranges[self.src][0] = max(valid_ranges[self.src][0], self.num + offset)

    def __repr__(self):
        return f"{self.src} {self.cmp} {self.num}"    
    
class Workflow:
    def __init__(self, rules):
        self.rule_pipe = [Rule(src, cmp, num, des) for src, cmp, num, des in re.findall(r"(\w+)(<|>)(\d+):(\w+)", rules)]
        self.default = rules.split(',')[-1]

    def match_condition(self,part):
        for rule in self.rule_pipe:
            if rule.match(part):
                return rule.des
        return self.default

def parse(filename):
    workflows, parts = open(filename).read().split("\n\n") 

    workflows_rules = {}
    for workflow in workflows.split('\n'):
        entry, rules = workflow.split('{')
        workflows_rules[entry] = Workflow(rules.strip('}'))

    parts = [{part: int(num) for part, num, in re.findall(r"(\w+)=(\d+)", line)} for line in parts.split('\n')]
    return workflows_rules, parts

def is_valid(part, workflows):
    des = "in"
    while des not in "AR":
        current = workflows[des]
        des = current.match_condition(part)
    return True if des == 'A' else False

def part1(workflows, parts):
    total = 0
    for part in parts:
        if is_valid(part,workflows):
            total += sum(part.values())
    print(total)


def change_min_max(rule, parts):
    for part in parts:
        rule.cmp_range(part)
    return parts

def get_valids(workflows, src, pos):
    if src == "A":
        return [{ch: [MIN,MAX] for ch in "xmas"}]
    if src == "R":
        return []
    # print(pos, workflows[src].rule_pipe)
    if pos == len(workflows[src].rule_pipe):
        return get_valids(workflows, workflows[src].default, 0)
    rule = workflows[src].rule_pipe[pos]
    valid = change_min_max(rule, get_valids(workflows, rule.des, 0) )
    not_valid = change_min_max(rule.create_opp(), get_valids(workflows, src, pos+1) )
    return valid + not_valid

def part2(workflows):
    valids = get_valids(workflows, "in", 0)
    total = 0
    for valid in valids:
        total += reduce(mul, [max - min + 1 for min, max in valid.values()])
    print(total)

def solution():
    filename = "./inputs/q19.txt"
    workflows, parts = parse(filename)
    part1(workflows, parts)
    part2(workflows)

solution()

# try 1
# def part2(workflows):
#     paths = [] 
#     stack = [("in", [])]

#     while stack:
#         src, path= stack.pop()

#         if src == 'A':
#             paths.append(path)
#             continue
        
#         if src == 'R':
#             continue

#         workflow = workflows[src]
        
#         opp_tracker = []
#         for rule in workflow.rule_pipe:
#             stack.append((rule.des, path + opp_tracker + [rule]))
#             opp_tracker.append(rule.create_opp())

#         stack.append((workflow.default, path + opp_tracker))    
    
#     total = 0
#     for path in paths:
#         valid_ranges = {key: [MIN,MAX]for key in ['x', 'm', 'a', 's']}
#         for rule in path:
#             rule.cmp_range(valid_ranges)
#         total += reduce(mul, [ max_num - min_num + 1 for min_num, max_num in valid_ranges.values()])
#     print(total)