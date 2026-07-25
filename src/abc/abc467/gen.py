import random

n = random.randint(3, 4)
print(n)
m = 10
print(m)
for i in range(n):
    print(random.randint(0, m - 1))
for i in range(n - 1):
    print(random.randint(0, m - 1))