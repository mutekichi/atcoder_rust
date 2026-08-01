import random

n = 10
print(n)

random_perm = [i + 1 for i in range(n)]
random.shuffle(random_perm)
print(*random_perm)