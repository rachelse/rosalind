#Enumerating Oriented Gene Orderings

def permutation(n, k):
    global permutations
    if k>=n:
        permutations.append(list(perm_list))
        # minus(perm_list)

    else:
        for i in range(n):
            if check_list[i]==0:
                check_list[i]=1
                perm_list[k]= num_list[i]
                permutation(n, k+1)
                check_list[i]=0
                perm_list[k]=0

def minus(n, m ):
    global signs
    if m >=n:
        signs.append(list(sign_list))

    else:
        sign_list[m] = 1
        minus(n,m+1)
        sign_list[m]=-1
        minus(n,m+1)

n = 5        
num_list = [i for i in range(1, n+1)]
check_list = [0]*n

perm_list = [0]*n
sign_list = [0]*n

permutations = []
signs = []

permutation(n,0)
minus(n, 0)

print(len(permutations)*len(signs))

for p in permutations:
    for s in signs:
        for i in range(len(p)):
            print(p[i]*s[i], end = ' ')
        print('')