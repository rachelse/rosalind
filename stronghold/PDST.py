#Creating a Distance Matrix
import numpy as np
def parse(question):
    data = open('/Users/steineggerlab/Downloads/rosalind_{}.txt'.format(question),'r')
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
    return strings

def PDST(seq):
    dist= np.empty((len(seq), len(seq)))

    for i in range(len(seq)):
        for j in range(len(seq)):
            if i==j:
                dist[i][j] = 0.0
            else:
                if not dist[i][j]:
                    mismatch =0
                    for k in range(len(seq[0])):
                        if seq[i][k]!=seq[j][k]:
                            mismatch +=1
                    prob = mismatch / len(seq[0])
                    dist[i][j], dist[j][i] = prob, prob 
    return dist

seq = parse('pdst')
dist = PDST(seq)
                
for i in range(len(seq)):
    for j in range(len(seq)):
        print(dist[i][j], end=' ')
    print('')