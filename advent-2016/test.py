n = 0
ll = list()
la = list()


with open("q3.txt") as f:
  ins = f.read()
  # print(ins)
  for l in ins.split('\n'):
      print(f"here - {l}")
      ll.append(list(map(int, l.split())))

  print(len(ll))
  for i in range(int(len(ll)/3)):
      for j in range(3):
          ln = [ll[i*3][j], ll[i*3+1][j], ll[i*3+2][j]]
          ln.sort()
          la.append(ln)
  print(len(la))
  for l in la:
      if l[0]+l[1] > l[2]:
          n += 1
# [401, 613, 725], [211, 312, 402], [215, 328, 520]]
  print(n)