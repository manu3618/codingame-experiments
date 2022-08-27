# https://www.codingame.com/ide/puzzle/unary

import sys
from itertools import groupby


def get_bloc(c):
    if c.startswith("1"):
        return "0 " + "0" * len(c)
    else:
        return "00 " + "0" * len(c)


message = input()

b = "".join([format(ord(c), "07b") for c in message])
print(b, file=sys.stderr, flush=True)

cahrs = ["".join(g) for _, g in groupby(b)]
print(cahrs, file=sys.stderr, flush=True)

print(" ".join(get_bloc(c) for c in cahrs))
