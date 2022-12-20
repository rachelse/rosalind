#Overlab Graphs

#parse data
data = open('/Users/steineggerlab/Desktop/lab/Rosalind/test/og.fasta', 'r')
data =data.readlines()
data_dict ={}

for i in range(0,len(data),3):
    add_data = data[i+1][:-1] + data[i+2][:-1]
    data_dict[data[i][1:-1]] = add_data

key_list = list(data_dict)
data_dict[key_list[0]][-3:]
#overlap graphs
for s in range(len(data_dict)):
    for t in range(len(data_dict)):
        if s == t:
            continue
        else: 
            if data_dict[key_list[s]][-3:] == data_dict[key_list[t]][:3]:
                print(key_list[s], key_list[t])