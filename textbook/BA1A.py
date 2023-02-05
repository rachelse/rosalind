#Compute the Number of Times a Pattern Appears in a Text

def BA1A(text, pattern):
    n = len(pattern)
    start = pattern[0]
    count =0
    for i in range(len(text)):
        if text[i:i+n] == pattern:
            count+=1
    return count

code = 'ba1a'
problem = open(f'/Users/steineggerlab/Downloads/rosalind_{code}.txt', 'r').readlines()
text, pattern = problem[0].strip(), problem[1].strip()

## test
# text, pattern = 'CGATATATCCATAG', 'ATA'

print(BA1A(text,pattern))