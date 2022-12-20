#Open Reading Frames
codon = {'TTT': 'F'   , 'CTT': 'L',  'ATT': 'I',  'GTT': 'V',
         'TTC': 'F'   , 'CTC': 'L',  'ATC': 'I',  'GTC': 'V',
         'TTA': 'L'   , 'CTA': 'L',  'ATA': 'I',  'GTA': 'V',
         'TTG': 'L'   , 'CTG': 'L',  'ATG': 'M',  'GTG': 'V',
         'TCT': 'S'   , 'CCT': 'P',  'ACT': 'T',  'GCT': 'A',
         'TCC': 'S'   , 'CCC': 'P',  'ACC': 'T',  'GCC': 'A',
         'TCA': 'S'   , 'CCA': 'P',  'ACA': 'T',  'GCA': 'A',
         'TCG': 'S'   , 'CCG': 'P',  'ACG': 'T',  'GCG': 'A',
         'TAT': 'Y'   , 'CAT': 'H',  'AAT': 'N',  'GAT': 'D',
         'TAC': 'Y'   , 'CAC': 'H',  'AAC': 'N',  'GAC': 'D',
         'TAA': 'Stop', 'CAA': 'Q',  'AAA': 'K',  'GAA': 'E',
         'TAG': 'Stop', 'CAG': 'Q',  'AAG': 'K',  'GAG': 'E',
         'TGT': 'C'   , 'CGT': 'R',  'AGT': 'S',  'GGT': 'G',
         'TGC': 'C'   , 'CGC': 'R',  'AGC': 'S',  'GGC': 'G',
         'TGA': 'Stop', 'CGA': 'R',  'AGA': 'R',  'GGA': 'G',
         'TGG': 'W'   , 'CGG': 'R',  'AGG': 'R',  'GGG': 'G'} 
base= {'A':'T', 'T':'A', 'C':'G', 'G':'C'}
bp =''
s ='TGATGAGCGTGAAAACAGACACTCTGTAACATCATCGTGCCGACCGGACGTTACTACGCGTTGAGAGAGTGAACTGGTCCAAGAGAGGCCATTATAAGGTTGGGACAAAAACATCAAGGGCTTACGGTGGCTCGAGTTTGTATCAATTCGAAGCCAACGGCTCCGTGGTTTCGCAACGATATGGCTAAAGTTCTTACATCTGATCTCCATCTTCCCGACGCGCACACTTGGATATTAGACGATAAGGGATACCACAGAGTCAGTACTATGTTTCCAGCCTGCATGAAAGAAGATTCTCGCACATTAGCCGACATCGACGCCAAGACACCCTGGCCGCATCTATGCTGGAGACAAAGTATTTGGTTAATGGTTGATAAGGTTTGCGTAAGGGCTCCTGCCGGGAGAATCCTATCCTCTGATGCAACACTCTTAGGAGACTGTAGCATAGCTATGCTACAGTCTCCTAAGAGTGTTGCATATAAGTACAATATACAGAACAGAACCTACACTGGGTTTAAGAATCGATGCTATTGATAATCAGTTATCGGCAACGCGCATCCCGCGGGTGCGGTGTAAAGCAGGGTTACATAACCGAAGTAAGAACGAAAACGACTACCGCGCTACATGACAGTATCCGTTCCCAAAGGCATGACCGCTTATGGGTCGGCAGAGCAAAATGTTCGCTAGTGGAAAGCTCCATGAGAAATACTCTGGATAATAGTCGCACCAACAGCCTAACTCGGCCATATACACGTTGAACAGAGCGCTAATTAAGCAAATAGTGGGAGCGCGTAAACTCTCCAGGCCACCATTTTAATCACAGTGGAATCGTGACGCCTAACGCAACTTTACATTAGCATGACGGGGGCTTCCCATCCCCATCGTGCAGCACTCGT'

for b in reversed(range(len(s))):
    bp += base[s[b]]
    

def get_orf(s):
    orfs =[]
    p = 0
    for i in range(len(s)-2):
        if s[i:i+3]=='ATG':
            orf=''
            for j in range(i, len(s)-2,3) :
                try:
                    if codon[s[j:j+3]]=='Stop':
                        orf+=codon[s[j:j+3]]
                        break
                except:
                    pass
                orf+=codon[s[j:j+3]]
            if 'Stop' in orf:
                orfs.append(orf[:-4])

    return orfs

orf_s = get_orf(s)
orf_bp = get_orf(bp)
orfs = set(orf_s +orf_bp)

    
for orf in orfs:
    print(orf)