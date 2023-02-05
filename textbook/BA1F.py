# BA1F: Find a Position in a Genome Minimizing the Skew

def BA1F(pwd):
    seq = open(pwd, 'r').readline().strip()

    g = 0
    c = 0
    skew = {}
    for n in range(len(seq)):
        skew[n] = g-c
        if seq[n] == 'G':
            g+=1
        elif seq[n] == 'C':
            c+=1
    
    # sort skew dictionary to get minimizing i
    sorted_skew = sorted(skew.items(), key= lambda item: item[1])
    min_val = sorted_skew[0][1]
    for (idx, val) in sorted_skew:
        if val <= min_val:
            print(idx, end = ' ')


sample_pwd = "/Users/steineggerlab/Downloads/rosalind_test.txt"
test_pwd = "/Users/steineggerlab/Downloads/rosalind_ba1f.txt"

# BA1F(sample_pwd)
BA1F(test_pwd)