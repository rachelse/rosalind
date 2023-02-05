#Finding a protein motif

from Bio import SeqIO

id_list = open('/Users/steineggerlab/Downloads/rosalind_mprt-2.txt', 'r')
id_list = id_list.readlines()
url = "https://rest.uniprot.org/uniprotkb/"

def get_NGS(trimmed_id):
    locations = []
    test = open('/Users/steineggerlab/Downloads/{}.fasta'.format(trimmed_id), 'r')

    fasta =list(SeqIO.parse(test, 'fasta'))
    seq =fasta[0].seq

    for i in range(len(seq)):
        try:
            if seq[i]=='N' and seq[i+1] !='P' and seq[i+3] !='P' and seq[i+2] in ['S', 'T']:
                locations.append(i+1)
        except: 
            pass
    return locations

for id in id_list:
    if '_' not in id:
        trimmed_id = id[:-1]
    else:
        trimmed_id = id.split('_')[0]

    id_url = url+trimmed_id+'.fasta'
    # os.system('wget --directory-prefix="/Users/steineggerlab/Downloads/" {}'.format(id_url))
    locations = get_NGS(trimmed_id)
    if len(locations) !=0:
        print(id[:-1])
        print(" ".join(str(i) for i in locations))