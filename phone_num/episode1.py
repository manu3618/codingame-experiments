# https://www.codingame.com/ide/puzzle/telephone-numbers
import math
import sys
from itertools import product

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.

phone_nums = []

n = int(input())
storage = 0
for _ in range(n):
    telephone = input()
    max_common_len = 0
    if any(telephone.startswith(x) or x.startswith(telephone) for x in phone_nums):
        # no more storage
        phone_nums.append(telephone)
        continue
    common_lengths = {
        idx
        for idx, num in product(range(len(telephone)), phone_nums)
        if telephone.startswith(num[:idx])
    }
    common_lengths.add(0)
    storage += len(telephone) - max(common_lengths)
    phone_nums.append(telephone)


# Write an answer using print
# To debug: print("Debug messages...", file=sys.stderr, flush=True)

# The number of elements (referencing a number) stored in the structure.
print(storage)
