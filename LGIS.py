#Longest Increasing Subsequence(2)
data = open('/Users/steineggerlab/Downloads/rosalind_lgis-3.txt', 'r')
data = data.readlines()

n = int(data[0])
seq =data[1]
seq = list(map(int, seq.split(' ')))

inc_track = [-1]*n
inc_num = [0]*n
inc_num[0]=1

inc_index = 0
max_inc = 1


for i in range(1, n):
    for j in range(0,i):
        if seq[j] < seq[i] and inc_num[i]<=inc_num[j]:
            inc_num[i] = inc_num[j]+1
            inc_track[i] = j
            if inc_num[i] > max_inc:
                max_inc = inc_num[i]
                inc_index = i

    if inc_num[i] == 0:
        inc_num[i] = 1


dec_track = [-1]*n
dec_num = [0]*n
dec_num[0]=1

dec_index = 0
max_dec = 1


for i in range(1, n):
    for j in range(0,i):
        if seq[j] > seq[i] and dec_num[i]<=dec_num[j]:
            dec_num[i] = dec_num[j]+1
            dec_track[i] = j
            if dec_num[i] > max_dec:
                max_dec = dec_num[i]
                dec_index = i

    if dec_num[i] == 0:
        dec_num[i] = 1

inc_answer = []

while inc_index != -1:
    inc_answer.append(seq[inc_index])
    
    inc_index = inc_track[inc_index]

for i in reversed(range(len(inc_answer))):
    print(inc_answer[i], end = ' ')
    
dec_answer = []

while dec_index != -1:
    dec_answer.append(seq[dec_index])
    dec_index = dec_track[dec_index]

print('')
for i in reversed(range(len(dec_answer))):
    print(dec_answer[i], end = ' ')