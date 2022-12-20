#Perfect matchings and rna secondary structures
seq = 'UCCGUAUCAGAUUCUUGGUACCGACGCGUACGACGCAACUGAACCUGUAUCACCUAUGUUGGACGGGCAGGAUCGA'
A,C = 0, 0
total = 1
for i in seq:
    if i=='A':
        A+=1
        total *= A
    elif i =='C':
        C+=1
        total *= C
print(total)
print(A, C)