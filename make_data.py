print(1)
n = 400
print(n)
for i in range(n):
    list = []
    for j in range(n):
        if i == 0 or j == 0:
            list.append(1)
        elif i == j:
            list.append(1)
        else:
            small, large = min(i, j), max(i, j)
            if small % 2 == 0:
                list.append(1)
            else:
                list.append(0)
    print(" ".join(map(str, list)))
# for i in range(n):
#     list = []
#     for j in range(n):
#         if i == j or i == 0 or j == 0:
#             list.append(1)
#         else:
#             list.append(0)
#     print(" ".join(map(str, list)))
