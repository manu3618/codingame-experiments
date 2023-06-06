# https://www.codingame.com/ide/puzzle/dwarfs-standing-on-the-shoulders-of-giants
import math
import sys

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.

n = int(input())  # the number of relationships of influence
for i in range(n):
    # x: a relationship of influence between two people (x influences y)
    x, y = [int(j) for j in input().split()]

# Write an answer using print
# To debug: print("Debug messages...", file=sys.stderr, flush=True)


# The number of people involved in the longest succession of influences
print("2")
