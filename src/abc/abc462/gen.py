import random

n = random.randint(1, 10)
d = random.randint(1, 10)
print(n)
print(d)
for _ in range(n):
    s = random.randint(1, 10)
    t = random.randint(1, 10)
    print(min(s, t), end=" ")
    print(max(s, t))
    