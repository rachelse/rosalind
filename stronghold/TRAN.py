#Transitions and Trnasversions

#seq 추출
data = open('/Users/steineggerlab/Downloads/rosalind_tran.txt','r')
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

s1 = strings[0]
s2 = strings[1]

transition = 0
transversion = 0
transitions = {'A':'G', 'G':'A', 'T':'C', 'C':'T'}
for i in range(len(s1)):
    if s1[i] == s2[i]:
        pass
    elif transitions[s1[i]] == s2[i]:
        transition +=1
    else:
        transversion+=1

print(transition/transversion)