#Genome Assembly as Shortest superstrng

#seq 추출
data = open('/Users/steineggerlab/Downloads/rosalind_long-6.txt','r')
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

#여기서부터 ₩
len_s = len(strings)
threshold = len(strings[0])//2

def calc_dist(i,j):
    global score
    string_i = strings[i]
    string_j = strings[j]
    ij,ji = 0,0
    for si in range(len(string_i)):
        if string_i[si:] == string_j[:len(string_i)-si]:
            ij = len(string_i)-si
            break
    
    for sj in range(len(string_j)):
        if string_j[sj:] == string_i[:len(string_j)-sj]:
            ji = len(string_j)-sj
            break

    if ij >=threshold:
        if ij > score_overlap[i]:
            score_overlap[i] = ij
            index_after[i] = j
            index_before[j] = i

    if ji >= threshold:
        if ji > score_overlap[j]:
            score_overlap[j]= ji
            index_after[j] = i
            index_before[i] = j
        
score_overlap = [0 for i in range(len_s)]
index_before = [-1 for i in range(len_s)]
index_after = [-1 for i in range(len_s)]
used = [0 for i in range(len_s)]

total_read = 0

for i in range(len_s):
    for j in range(i+1, len_s):
        result = calc_dist(i,j)

for index in range(len_s):
    if index_before[index]==-1:
        start = index

    if index_after[index]==-1:
        end = index

superstring=''

while total_read<len_s:
    superstring += strings[start][:-1*score_overlap[start]]
    start = index_after[start]
    total_read+=1

superstring+=strings[end]


print(superstring)