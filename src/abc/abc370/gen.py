import random

n = random.randint(1, 10)
a = [random.randint(1, 100) for _ in range(n)]

print(n)
print(*a)
