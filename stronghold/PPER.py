#Partial permatations
n , k = 21, 7
perm = 1
for i in range(k):
    perm *= n 
    n-=1

print(perm%1000000)