#Matching Random Motifs
data = open('/Users/steineggerlab/Downloads/rosalind_rstr.txt', 'r').readlines()
N = int(data[0].split(' ')[0])
x = float(data[0].split(' ')[1][:-1])
s = data[1][:-1]

def RSTR(N, x, s):
    gc = x/2
    at = 0.5-gc
    prob = 1
    for base in s:
        if base in ['G','C']:
            prob*=gc
        else:
            prob*=at
    print(1- (1-prob)**N)

RSTR(N,x,s)