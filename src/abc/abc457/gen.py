import random

n = random.randint(1, 10)
m = random.randint(2, 10)
print(n)
print(m)
for _ in range(m):
    l = random.randint(1, n)
    r = random.randint(1, n)
    low = min(l, r)
    high = max(l, r)
    print(low, high)

q = random.randint(1, 5)
print(q)

for _ in range(q):
    l = random.randint(1, n)
    r = random.randint(1, n)
    low = min(l, r)
    high = max(l, r)
    print(low, high)
    
