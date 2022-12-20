#Mendel's First Law
a, b, c, = 27, 30, 19

total_case = (a+b+c)*(a+b+c-1)
b_b = b*(b-1)/4
b_c = b*c
c_c = c*(c-1)
probability = 1 - (b_b + b_c + c_c)/total_case
print(probability)