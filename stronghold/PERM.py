#Enumerating gene orders

def run(turn,f):

    if turn >= n :
        store = list(map(str,perm))
        store = ' '.join(store)
        result.append(store)

    else:
        for p in range(n):
            if check[p]==0:
                check[p] = 1
                perm[turn] = p+1
                run(turn+1,f)
                check[p] = 0
                perm[turn] = 0


result= []
f = 0
while f<n:
    n = 7
    perm = [0]*n
    check = [0]*n

    perm[0]=f+1
    check[f]=1
    run(1,f)

    f+=1
print(len(result))
print('\n'.join(i for i in result))