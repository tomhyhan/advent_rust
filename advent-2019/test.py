import itertools
from collections import defaultdict

class Program(object):
    def __init__(self, pid, program_file, input):
        self.P = defaultdict(int)
        for i,x in enumerate(open(program_file).read().split(',')):
            self.P[i] = int(x)
        self.input = input
        self.ip = 0
        self.pid = pid
        self.rel_base = 0
        self.halted = False
    def idx(self, i, I):
        mode = (0 if i>=len(I) else I[i])
        val = self.P[self.ip+1+i]
        if mode == 0:
            pass # no-op
        elif mode == 2:
            val = val+self.rel_base
        else:
            assert False, mode
        return val
    def val(self, i, I):
        mode = (0 if i>=len(I) else I[i])
        val = self.P[self.ip+1+i]
        if mode == 0:
            val = self.P[val]
        elif mode == 2:
            val = self.P[val+self.rel_base]
        return val
    def run_all(self):
        ans = []
        while True:
            val = self.run()
            if val == None:
                return ans
            ans.append(val)

    def run(self):
        """Return next output"""
        while True:
            cmd = str(self.P[self.ip])
            opcode = int(cmd[-2:])
            I = list(reversed([int(x) for x in cmd[:-2]]))
            if opcode == 1:
                i1,i2 = self.val(0,I),self.val(1,I)
                self.P[self.idx(2, I)] = self.val(0, I) + self.val(1, I)
                self.ip += 4
            elif opcode == 2:
                i1,i2 = self.val(0,I),self.val(1,I)
                self.P[self.idx(2, I)] = self.val(0, I) * self.val(1, I)
                self.ip += 4
            elif opcode == 3:
                print("here")
                inp = self.input()
                self.P[self.idx(0, I)] = inp #self.Q[0]
                #self.Q.pop(0)
                self.ip += 2
            elif opcode == 4:
                ans = self.val(0, I)
                self.ip += 2
                return ans
            elif opcode == 5:
                self.ip = self.val(1, I) if self.val(0, I)!=0 else self.ip+3
            elif opcode == 6:
                self.ip = self.val(1, I) if self.val(0, I)==0 else self.ip+3
            elif opcode == 7:
                self.P[self.idx(2, I)] = (1 if self.val(0,I) < self.val(1,I) else 0)
                self.ip += 4
            elif opcode == 8:
                self.P[self.idx(2, I)] = (1 if self.val(0,I) == self.val(1,I) else 0)
                self.ip += 4
            elif opcode == 9:
                self.rel_base += self.val(0, I)
                self.ip += 2
            else:
                assert opcode == 99, opcode
                self.halted = True
                return None


# G = defaultdict(int)
def get_input():
    paddle= 0
    ball= 0
    for pos,v in G.items():
        if v == 3:
            paddle = pos
        if v == 4:
            ball = pos

    if ball[1]<paddle[1]:
        return -1
    elif ball[1]>paddle[1]:
        return 1
    else:
        return 0

# P = Program('0', 'test.txt', get_input)
# while not P.halted:
#     x = P.run()
#     y = P.run()
#     c = P.run()
#     if x==-1 and y==0:
#         score = c
#     G[(y,x)] = c
# ans = 0
# for k,v in G.items():
#     if v == 2:
#         ans += 1
# print(ans)
	
G = defaultdict(int)
P = Program('0', 'test.txt', get_input)
# get_input()
P.P[0] = 2
score = 0
while not P.halted:
    x = P.run()
    # print(x)
    y = P.run()
    # print(y)
    c = P.run()
    # print(c)
    if x==-1 and y==0:
        score = c
    G[(y,x)] = c
    # break
print(score)
