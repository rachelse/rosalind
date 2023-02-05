# Compute the Probability of a Hidden Path

def BA10A(seq, matrix):
    prob = 0.5
    for s in range(1,len(seq)):
        this = seq[s]
        prev = seq[s-1]
        prob*=matrix[this][prev]
    return prob
    
seq = 'ABABBAABABBABBBBBABBAAABBABBAABAABAAABBBAAAAAAABAB'
matrix = {'A':{'A':0.145, 'B':0.321}, 'B':{'A':0.855, 'B':0.679}}
print(BA10A(seq,matrix))