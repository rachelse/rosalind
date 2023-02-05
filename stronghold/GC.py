#Computing GC content
from ast import operator
import os

data_open = open('/Users/steineggerlab/Downloads/rosalind_gc.txt', 'r')
data = data_open.read()
data = data.split('>')[1:]
data_dict = {}

for i in data:
    sample , sequence = i.split('\n')[0], i.split('\n')[1:]
    sequence = ''.join(sequence)
    count = 0
    for s in sequence:
        if s in ['C','G']:
            count +=1
    count = count/len(sequence)*100
    data_dict[sample]= count

sorted_dict = sorted(data_dict.items(), key= lambda x: x[1], reverse=True)
print(sorted_dict[0][0])
print(sorted_dict[0][1])