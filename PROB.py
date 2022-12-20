#Introduction to Random Strings
from math import log10
data= open('/Users/steineggerlab/Downloads/rosalind_prob_1_dataset.txt', 'r').readlines()
seq = data[0][:-1]
arrA = data[1][:-1].split(' ')
arrA = list(map(float, arrA))
arrB = []

for p in arrA:
    gc = p/2
    at = 0.5-gc
    prob = 1
    for s in seq:
        if s in ['G', 'C']:
            prob*=gc
        else:
            prob*=at
    
    arrB.append(log10(prob))
for b in arrB:
    print(b, end=' ')