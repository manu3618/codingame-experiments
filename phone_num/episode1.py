# https://www.codingame.com/ide/puzzle/telephone-numbers
import math
import sys

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.

phone_nums = []

n = int(input())
storage = 0
to_break = False


def get_max_common_len(a: str, b: str):
    for idx in range(len(a) + 1, 0, -1):
        if a[:idx] == b[:idx]:
            return idx
    return 0


for _ in range(n):
    telephone = input()
    if any(telephone.startswith(x) or x.startswith(telephone) for x in phone_nums):
        phone_nums.append(telephone)
        continue

    c = [get_max_common_len(num, telephone) for num in phone_nums]
    c.append(0)
    m = max(c)

    print("Debug messages... to add ", m, file=sys.stderr, flush=True)
    storage += len(telephone) - m
    print("Debug messages... current", storage, file=sys.stderr, flush=True)
    phone_nums.append(telephone)


print(storage)
