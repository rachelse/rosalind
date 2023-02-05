#Catalan Numbers and RNA Secondary structures
sequence = 'AUAU'
test = 'UAGCGUGAUCAC'
#seq 추출
data = open('/Users/steineggerlab/Downloads/rosalind_cat.txt','r')
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


def CAT(seq, mem = {}):
    bp = {'A':'U', 'C':'G', 'U':'A', 'G':'C'}
    length = len(seq)

    if length%2 ==1:
        return 0
    elif length ==0 :
        return 1
    elif seq in mem.keys():
        return mem[seq]
    elif length ==2 and bp[seq[0]]==seq[1]:
        return 1
    else:
        total = 0
        for i in range(1,length):
            if bp[seq[0]] == seq[i]:
                left = CAT(seq[1:i])
                right = CAT(seq[i+1:] )
                mem[seq[1:i]] = left
                mem[seq[i+1:]] = right
                total += (left*right)
    return total

print(CAT(string) % 1000000)