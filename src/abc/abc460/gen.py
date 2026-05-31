import random
h = random.randint(2, 5) 
w = random.randint(2, 5) 
print(h)
print(w)

for _ in range(h):
    for i in range(w):
        if random.randint(1, 2) == 1:
            print('.', end='')
        else:
            print('#', end='')
    print()
    