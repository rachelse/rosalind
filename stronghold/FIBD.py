#Mortal Fibonacci Rabbits
n, m = 95, 17


new = [1,0]
rabbits = [1,1]
for i in range(2,n):   
    generate = rabbits[-1]-new[-1]
    new.append(generate)
    if i>=m:
        rabbits.append(rabbits[-1]+generate-new[-m-1])
    else:
        rabbits.append(rabbits[-1]+new[-1])


print(rabbits[-1])