import numpy as np

def solution():
    # 0, 1, 2
    coeffs = np.array([[0,0,1], [1,1,1], [4,2,1]])
    # 65, 65 + 131, 65 + 131 * 2
    results = np.array([3725, 32896, 91055])
    s = np.linalg.solve(coeffs, results)
    print(s)
    # 26501365 = t * 131 + 65
    t =  (26501365 - 65) / 131
    print(int(s[0]*t*t+ s[1]*t + s[2]))

solution()
