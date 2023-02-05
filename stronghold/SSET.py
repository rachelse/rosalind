#Counting Subsets
def SSET(n):
    return (2**n)%1000000

n = open('/Users/steineggerlab/Downloads/rosalind_sset.txt','r').readlines()[0][:-1]
n = int(n)
print(SSET(n))