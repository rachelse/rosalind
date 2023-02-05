# BA1G: Compute the Hamming Distance Between Two Strings

def BA1G(pwd):
    file =  open(pwd, 'r').readlines()
    query, target = file[0].strip(), file[1].strip()

    dist = 0
    for i in range(len(query)):
        if query[i] != target[i]:
            dist+=1

    print(dist)


sample = '/Users/steineggerlab/Downloads/rosalind_test.txt'
test = '/Users/steineggerlab/Downloads/rosalind_ba1g.txt'

# BA1G(sample)
BA1G(test)