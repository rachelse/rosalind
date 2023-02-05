#Locating Restriction Sites
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

seq = parse('revp')[0]

def REVP(seq):
    output = []
    base = {'A':'T', 'C':'G', 'G':'C', 'T':'A'}

    for i in range(1,len(seq)-2):
        if seq[i]==base[seq[i+1]] and seq[i-1]==base[seq[i+2]]:
            output.append([i,4])
            if i-2>=0 and i+3<len(seq) and seq[i-2]==base[seq[i+3]]:
                output.append([i-1,6])
                if i-3>=0 and i+4<len(seq) and seq[i-3]==base[seq[i+4]]:
                    output.append([i-2,8])
                    if i-4>=0 and i+5<len(seq) and seq[i-4]==base[seq[i+5]]:
                        output.append([i-3,10])
                        if i-5>=0 and i+6<len(seq) and seq[i-5]==base[seq[i+6]]:
                            output.append([i-4,12]) 
    return output

ans = REVP(seq)
for a in ans:
    print(a[0], a[1])