#Independent Alleles

k, n = 6,18
total = 1
organism= 2**k

def combination(organism, i):
    init = 1
    for ii in range(1,i+1):
        init*=(organism-ii+1)/(ii)
    return init

for i in range(n):
    total -= (3/4)**(organism-i)*(1/4)**(i)*combination(organism,i)
print(total)