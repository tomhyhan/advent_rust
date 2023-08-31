
def calc_fib(n):
    memo = {0:0, 1:1}
    def fib(n):
        if n in memo:
            return memo[n]
        current = fib(n-1) + fib(n-2)     
        memo[n] = current
        return current
    return fib(n)
def fib(a,b,idx,n):
    if idx == n:
        return a
    return fib(b,a+b,idx+1,n)
# 0 1 1 2 3 5 
# 0 1 1 2 3 
print(calc_fib(5))
print(fib(0,1,0,1000))