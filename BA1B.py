# Find the Most Frequent Words in a String

def BA1B(text, k):
    kmer_dict = {}

    #kmer save
    for i in range(0, len(text)+1-k):

        if text[i:i+k] in kmer_dict.keys():
            kmer_dict[text[i:i+k]] +=1
        else:
            kmer_dict[text[i:i+k]] = 1

    sorted_dict = sorted(kmer_dict.items(), key=lambda x: x[1], reverse=True)
    max_v = 0
    kmers = []
    for k, v in sorted_dict:
        if v>=max_v:
            max_v = v
            kmers.append(k)
        else:
            break

    return kmers

code = 'ba1b'
problem = open(f'/Users/steineggerlab/Downloads/rosalind_{code}.txt', 'r').readlines()
text, k = problem[0].strip(), int(problem[1].strip())

# text, k = 'ACGTTGCATGTCGCATGATGCATGAGAGCT', 4

for kmer in BA1B(text, k):
    print(kmer, end = ' ')