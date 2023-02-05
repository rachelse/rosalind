#Finding a shared motif

from Bio import SeqIO

input_file = open('/Users/steineggerlab/Downloads/rosalind_lcsm.fasta')
records = list(SeqIO.parse(input_file, 'fasta'))

len_records = len(records)
min_len = 0
min_seq = 0

for i in range(len_records):
    if min_len ==0:
        min_len = len(records[i].seq)
        min_seq = i
    else:
        if min_len > len(records[i].seq):
            min_seq = i
            min_len = len(records[i].seq)

def check_motif(i, motif_len):
    for j in range(len_records):
        if standard[i:i+motif_len] not in records[j].seq:
            return False
    return True


i= 0
motif_len = 2
motif = ''
standard= records[min_seq].seq
while i+motif_len<min_len:
    if check_motif(i, motif_len) is True :
        motif = standard[i:i+motif_len]
        motif_len+=1
    else:
        i+=1

print(motif)
print(len(motif))
print(motif_len)