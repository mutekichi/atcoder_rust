import random

alphabets = [chr(i) for i in range(ord('A'), ord('Z') + 1)]

n = 10
numbers = [random.randint(0, 9) for _ in range(n)]
letters = [random.choice(alphabets) for _ in range(n)]

print(n)
print(" ".join(map(str, numbers)))
print(" ".join(letters))
