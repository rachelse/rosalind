#Introduction to Alternative Splicing
def comb(n,k):
    numer= 1
    denom =1
    turn =0
    for i in range(k):
        numer*=(n-turn)
        denom*=(k-turn)
        turn+=1
    return numer//denom

def ASPC(n,m):
    result = 0
    for k in range(m, n+1):
        result+=comb(n,k)
    return result

test = open('/Users/steineggerlab/Downloads/rosalind_aspc-2.txt', 'r').read()
n, m = int(test.split(' ')[0]), int(test.split(' ')[1]) 

# print(ASPC(6,3))
print(ASPC(n, m)%1000000)