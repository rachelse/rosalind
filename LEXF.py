#Enumerating k-mers Lexicographically
symbol='A B C D E F G H'.split(' ')
n=3
symbol

def permutation(turn):
    global string
    if turn >=n:
        print(string)
    else:
        for i in symbol:
            string+=i
            permutation(turn+1)
            string = string[:-1]
string = ''
permutation(0)