#Error Correction in Reads

#seq 추출
data = open('/Users/steineggerlab/Downloads/rosalind_corr.txt','r')
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

bp = {'A':'T', 'C':'G', 'T':'A', 'G':'C'}
strings_bp = {}
for string in strings:
    string_bp = ''
    for s in string[::-1]:
        string_bp+=bp[s]
    if string in strings_bp.keys() :
        strings_bp[string][1] += 1
        if string_bp in strings_bp.keys():
            strings_bp[string_bp][1] += 1
            strings_bp[string][1] +=1
    elif string_bp in strings_bp.keys():
        strings_bp[string_bp][1]+=1
        strings_bp[string] = [string_bp, 2]
    else:
        strings_bp[string] = [string_bp, 1]
reference =[]
target = []
for string, (string_bp, time) in strings_bp.items():
    if time >1:
        reference.extend([string, string_bp])
    if time ==1:
        target.append(string)

for t in target:
    for substitute in reference:
        score = 0
        for s in range(len(substitute)):
            if t[s] != substitute[s]:
                score+=1
        if score ==1:
            print('{}->{}'.format(t, substitute))
            break