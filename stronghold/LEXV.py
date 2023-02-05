#Ordering Strings of Varying Length Lexicographically
# symbols = 'D N A'
# n = 3

file = open('/Users/steineggerlab/Downloads/rosalind_lexv.txt', 'r').readlines()
symbols= file[0][:-1]
n = int(file[1][:-1])
sym_list = symbols.split(' ')

def LEXV(n, string):
    if len(string)>=n:
        print(string)

    elif len(string) ==0:
        for i in range(len(sym_list)):
            LEXV(n,sym_list[i])
    else:
        print(string)
        for i in range(len(sym_list)):
            LEXV(n, string+sym_list[i])

LEXV(n,'')