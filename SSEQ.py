#Finding a spliced motif

#seq 추출
data = open('/Users/steineggerlab/Downloads/rosalind_sseq-3.txt','r')
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

s = strings[0]
t = strings[1]


indices = [0 for i in range(len(t)+1)]

for i in range(len(t)):
    for j in range(indices[i], len(s)):
        if t[i] == s[j]:
            indices[i+1]= j+1
            break
        else:
            continue

for index in range(1, len(indices)):
    print(indices[index], end = ' ')