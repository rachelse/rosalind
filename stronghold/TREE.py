#Completing a Tree
file = open('/Users/steineggerlab/Downloads/rosalind_tree-2.txt', 'r')
file = file.readlines()
n = int(file[0])
counts = [0 for i in range(n)]
checks = [0 for i in range(n)]
adjacents= [[] for i in range(n)]
cluster = 0 

for nodes in file[1:]:
    n1, n2 = map(int, nodes.split(' '))
    counts[n1 -1] += 1
    counts[n2 -1] += 1
    adjacents[n1-1].append(n2-1)
    adjacents[n2-1].append(n1-1)

def internal(i):
    for num in adjacents[i]:
        if checks[num]==0:
            checks[num] = 1
            internal(num)

for i in range(n):
    if counts[i]==0:
        cluster+=1
        checks[i] =1
    elif counts[i]>=1:
        if checks[i]==0:
            internal(i)
            cluster+=1

print(cluster-1)