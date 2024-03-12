# https://www.codingame.com/ide/puzzle/flood-fill-example

import sys
from itertools import product

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.

w = int(input())
h = int(input())
lines = []
for i in range(h):
    lines.append(list(input()))


def step(grid):
    dims = len(grid), len(grid[0])
    new_grid = []
    for _ in range(dims[0]):
        new_grid.append(list("." * dims[1]))

    for x, y in product(range(dims[0]), range(dims[1])):
        new_grid[x][y] = grid[x][y]
        if grid[x][y] == "#":
            # '#'
            continue
        if grid[x][y] not in ".#":
            # I.D. to keep
            continue

        n = set(get_neighbors(grid, x, y))
        n.difference_update(set(".#"))
        if "+" in n:
            new_grid[x][y] = "+"
        elif len(n) == 1:
            new_grid[x][y] = list(n)[0]
        elif len(n) > 1:
            new_grid[x][y] = "+"

    return new_grid


def get_neighbors(grid, x, y):
    n = []
    if x > 0:
        n.append(grid[x - 1][y])
    if x < len(grid) - 1:
        n.append(grid[x + 1][y])
    if y > 0:
        n.append(grid[x][y - 1])
    if y < len(grid[0]) - 1:
        n.append(grid[x][y + 1])
    return n


def modify_id(grid):
    idx = 0
    new_grid = [list("." * len(grid[0])) for _ in grid]
    for x, y in product(range(len(grid)), range(len(grid[0]))):
        if grid[x][y] in ".#":
            new_grid[x][y] = grid[x][y]
        else:
            new_grid[x][y] = grid[x][y] + str(idx)
            idx += 1
    return new_grid


def reset_id(grid):
    new_grid = [list("." * len(grid[0])) for _ in grid]
    for x, y in product(range(len(grid)), range(len(grid[0]))):
        new_grid[x][y] = grid[x][y][0]
    return new_grid


lines = modify_id(lines)
for idx in range(1000):
    print(idx, file=sys.stderr, flush=True)
    print("\n".join(" ".join(line) for line in lines), file=sys.stderr, flush=True)
    print(idx, file=sys.stderr, flush=True)
    new_lines = step(lines)
    if new_lines == lines:
        break
    lines = new_lines

lines = reset_id(lines)
print("\n".join("".join(line) for line in lines))
