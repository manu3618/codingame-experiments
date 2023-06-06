# https://www.codingame.com/ide/puzzle/dwarfs-standing-on-the-shoulders-of-giants
import sys
from itertools import product

paths = []


def update_path(paths, x, y):
    """Add y to each path finishing by x"""
    for path in paths:
        print("path", path, file=sys.stderr)
        if path[-1] == x:
            print("path", path, y, file=sys.stderr)
            paths.append(path + [y])
        if path[0] == y:
            print("path", x, path, file=sys.stderr)
            paths.append([x] + path)
    paths.append([x, y])


def join_disjoints(paths):
    for path0, path1 in product(paths, paths):
        if path0[-1] == path1[0]:
            print("joining", path0, path1, file=sys.stderr)
            paths.append(path0 + path1[1:])


n = int(input())  # the number of relationships of influence
for _ in range(n):
    x, y = [int(x) for x in input().split()]
    print("____", x, y, "____", file=sys.stderr)
    update_path(paths, x, y)

join_disjoints(paths)

print(max(len(path) for path in paths))
