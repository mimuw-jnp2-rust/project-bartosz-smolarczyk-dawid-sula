#%%
values = [(40, 400), (50, 375), (60, 325), (70, 250), (80, 200), (90, 175)]

for _ in range(5): 
    n = len(values)
    values_new = [values[0]]

    for i in range(n - 1):
        left = values[i]
        right = values[i + 1]
        arg = left[0] + (right[0] - left[0]) * (n - i - 1) / n
        value = left[1] + (right[1] - left[1]) * (n - i - 1) / n
        values_new.append((arg, value))
    values_new.append(values[len(values) - 1])
    values = values_new

print(values)
for (arg, val) in values:
    print(f"[{arg}, {val}],")
# %%
