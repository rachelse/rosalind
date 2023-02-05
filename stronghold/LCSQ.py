#Finding a Shared Spliced Motif
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

def LCSQ(s1, s2):
    shared = []
    for _ in range(len(s1)+1):
        lst = [0]*(len(s2)+1)
        shared.append(lst)

    # max_value = 0
    # max_index = 0
    #fill the best match
    for i in range(1, len(s1)+1):
        for j in range(1, len(s2)+1):
            if s1[i-1]==s2[j-1]:
                shared[i][j] = shared[i-1][j-1]+1
                # if shared[i][j]>max_value:
                #     max_value = shared[i][j]
            else:
                shared[i][j] = max(shared[i][j-1], shared[i-1][j])

    lcsq = ''
    x, y = len(s1), len(s2)
    while x*y !=0:
        if shared[x][y] == shared[x-1][y]:
            x-=1
        elif shared[x][y]==shared[x][y-1]:
            y-=1
        else:
            lcsq =s1[x-1]+lcsq
            x-=1
            y-=1

    return lcsq

s = 'AACCTTGG'
t = 'ACACTGTGA'
seqs = parse('lcsq-3')
# print(LCSQ(s,t))
print(LCSQ(seqs[0], seqs[1]))