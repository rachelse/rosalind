# FIBO: Fibonacci Numbers

def FIBO(n):
    n0, n1 = 0, 1
    for i in range(1,n):
        n0, n1 = n1, n0+n1
    print(n1)

FIBO(24)