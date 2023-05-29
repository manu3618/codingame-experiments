# https://www.codingame.com/ide/puzzle/mayan-calculation
import math
import sys

import numpy as np


def to_base_n(x: int, n=20):
    res = []
    while x:
        res.append(x % n)
        x = x // n
    return res[::-1]


l, h = [int(i) for i in input().split()]
all_nums = []
maya_to_dec = {}
dec_to_maya = {}
for _ in range(h):
    all_nums.append(input())

for n in range(20):
    representation = "\n".join([all_nums[i][(l * n) : l * (n + 1)] for i in range(h)])
    maya_to_dec[representation] = n
    dec_to_maya[n] = representation

s1 = int(input())
num1 = "\n".join(input() for _ in range(s1))
s2 = int(input())
num2 = "\n".join(input() for _ in range(s2))

num1 = maya_to_dec[num1]
num2 = maya_to_dec[num2]

operation = input()

result = eval(f"{num1} {operation} {num2}")


print("Debug messages...", result, file=sys.stderr, flush=True)
result = to_base_n(result)
print("Debug messages...", result, file=sys.stderr, flush=True)


# Write an answer using print
# To debug: print("Debug messages...", file=sys.stderr, flush=True)
# lines = ["".join([dec_to_maya[c][i] for c in result]) for i in range(l)]
print("\n".join(dec_to_maya[c] for c in result))
