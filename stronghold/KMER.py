#k-Mer Composition
#seq 추출
data = open('/Users/steineggerlab/Downloads/rosalind_kmer.txt','r')
data = data.readlines()

i= 0
strings = []

while i < len(data):
    if data[i].startswith('>'):
        if i != 0 :
            strings.append(string)
        string = ''
        i+=1

    else:
        string += data[i].strip()
        i+=1
strings.append(string)
############################
k_mer_dict = {}

base = ['A','C', 'G', 'T']

def permutation(n,k):
    global k_mer
    if k >= n:
        k_mer_dict[k_mer] = 0
    else :
        for i in range(4):
            k_mer += base[i]
            permutation(n,k+1)
            k_mer = k_mer[:-1]
for i in range(4):
    k_mer = base[i]
    permutation(4,1)

for start in range(4):
    for i in range(start,len(string),4):
        k_mer = string[i:i+4]
        if len(k_mer)==4:
            k_mer_dict[k_mer] += 1

for value in k_mer_dict.values():
    total+=value
    print(value, end =' ')