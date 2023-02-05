#Consensus and Profile

input = open('/Users/steineggerlab/Downloads/rosalind_cons-2.txt', 'r')
input =input.read()
seq_list = input.split('>')[1:]


seq_num = len(seq_list)
seq_len = len(seq_list[0])
consensus = {'A':[], 'T':[], 'C':[], 'G':[]}
consensus_seq = ''
for l in range(seq_len):
    con = {'A':0, 'T':0, 'C':0, 'G':0}
    for s in seq_list:
        if s[l] =='A':
            con['A']+=1
        elif s[l] =='T':
            con['T']+=1
        elif s[l] =='C':
            con['C']+=1
        elif s[l] == 'G':
            con['G']+=1
        else:
            continue
    if con['A']+con['T']+con['C']+con['G'] != 0:
        consensus['A'].append(con['A']) 
        consensus['T'].append(con['T']) 
        consensus['C'].append(con['C']) 
        consensus['G'].append(con['G'])
        consensus_seq+=max(con, key = lambda x : con[x])
    else:
        continue

print(consensus_seq)
for c in consensus:
    print(c+':', end=' ')
    for i in consensus[c]:
        
        print(i, end=' ')
    print('')