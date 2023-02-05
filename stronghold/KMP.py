#Speeding Up Motif Finding
#seq 추출
data = open('/Users/steineggerlab/Downloads/rosalind_kmp-2.txt','r')
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
len_s = len(string)
P = [0]*len_s

for i in range(2,len_s+1):
    substring = string[:i]

    for j in range(P[i-2]+1, 0, -1):
        if substring[:j] == substring[len(substring)-j:len(substring)] :
            P[i-1]=j
            break

for p in P:
    print(p, end = ' ')