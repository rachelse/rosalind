#Calculating Expected Offsprint

a= [18521, 19836, 18017, 16274, 18975, 17477]

p = [1, 1, 1, 0.75, 0.5, 0]

total = 0

for i in range(6):
    total += 2*a[i]*p[i]

print(total)