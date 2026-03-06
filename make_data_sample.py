print(2000)
print(2000)
for i in range(2000):
    for j in range(2000):
        print((i * 29486 + j * 3059) % 1000, end=" ")
    print()