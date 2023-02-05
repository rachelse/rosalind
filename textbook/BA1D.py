# BA1D: Find All Occurences of a Pattern in a string
def BA1D(pwd):
    file = open(pwd).readlines()
    pattern, genome = file[0].strip(), file[1].strip()

    match = []

    for i in range(len(genome)-len(pattern)+1):
        if pattern == genome[i:i+len(pattern)]:
            print(i, end = ' ')
    


sample_pwd = "/Users/steineggerlab/Downloads/rosalind_test.txt"
test_pwd = "/Users/steineggerlab/Downloads/rosalind_ba1d.txt"

# BA1D(sample_pwd)
# BA1D(test_pwd)
