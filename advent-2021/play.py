def fib(n):
    # if n == 0 :
    #     return 0
    # if n == 1:
    #     return 1
    if n + 1 <= 1:
        return n
    return fib(n-1) + fib(n-2)

print(fib(3))