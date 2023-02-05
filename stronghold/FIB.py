#Rabbits and Reccurence Relations
n , k = 31, 4

n_2 = 1
n_1 = 1

if n == 1 or n ==2:
    print(1)
else:
    for i in range(3,n+1):
        n_2 , n_1 = n_1, n_1+n_2*k
    print(n_1)