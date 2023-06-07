# https://www.codingame.com/ide/puzzle/telephone-numbers
import math
import sys

prefixes = set()

n = int(input())
storage = 0


for _ in range(n):
    telephone = input()
    new_pref = {
        y
        for idx in range(1, len(telephone) + 1)
        if (y := telephone[:idx]) not in prefixes
    }
    storage += len(new_pref)
    prefixes.update(new_pref)


print(storage)
