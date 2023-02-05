# Find the Reverse Complement of a String
def BA1C(seq):
    base = {'A':'T', 'C':'G', 'G':'C', 'T':'A'}
    complementary =''
    for s in seq[::-1]:
        complementary+=base[s]
    return complementary

# seq= 'AAAACCCGGT'
code = 'ba1c'
seq= open(f'/Users/steineggerlab/Downloads/rosalind_{code}.txt', 'r').readline().strip()
print(BA1C(seq))