# BA1E: Find Patterns Forming Clumps in a String

def BA1E(pwd):
    file = open(pwd).readlines()
    genome = file[0].strip()
    L = len(genome)
    k, l, t = map(int,file[1].strip().split(' '))
    clumps = {}

    for i in range(L-l+1): #starting index of each clump
        lk_clump = {}

        for j in range(i, i+l-k+1): #look into the clump

                if genome[j:j+k] in lk_clump.keys() :
                    lk_clump[genome[j:j+k]] +=1
                else:
                    lk_clump[genome[j:j+k]] =1


        for key, val in lk_clump.items():
            if val>=t:
                clumps[key] = 0

    for kmer in clumps.keys():
        print(kmer, end = ' ')

sample_pwd = "/Users/steineggerlab/Downloads/rosalind_test.txt"
test_pwd = "/Users/steineggerlab/Downloads/rosalind_ba1e.txt"

# BA1E(sample_pwd)
BA1E(test_pwd)