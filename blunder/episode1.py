# https://www.codingame.com/ide/puzzle/blunder-episode-1
import sys

import numpy as np

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.


def which(arr, c="@"):
    for idx, row in enumerate(arr):
        if c in row:
            return idx, row.index(c)


def steps(arr, pos, teleporters=()):
    pos = np.array(pos)
    priorities = ["SOUTH", "EAST", "NORTH", "WEST"]
    depl = {"SOUTH": (1, 0), "EAST": (0, 1), "NORTH": (-1, 0), "WEST": (0, -1)}
    cur_dir = "SOUTH"
    dirs = {w[0]: w for w in priorities}
    cass = False
    dir_idx = 0
    path = []
    while True:
        next_pos = pos + depl[cur_dir]
        next_case = arr[next_pos[0]][next_pos[1]]
        cur_case = arr[pos[0]][pos[1]]
        print(
            "Debug messages... begining",
            pos,
            next_pos,
            depl[cur_dir],
            next_case,
            cur_case,
            file=sys.stderr,
            flush=True,
        )

        if any(np.all(pos == p) for p in path[:-1]):
            print("Debug messages... loop", path, file=sys.stderr, flush=True)
            return "LOOP"
        if cur_case == "$":
            print("Debug messages... real end", file=sys.stderr, flush=True)
            return cur_dir
        if cur_case in " @":
            pass
        if cur_case in "SENW":
            if cur_dir != dirs[cur_case]:
                cur_dir = dirs[cur_case]
                continue
            cur_dir = dirs[cur_case]
        if cur_case == "I":
            priorities = priorities[::-1]
        if cur_case == "X" and cass:
            arr[pos[0]][pos[1]] = " "
        if cur_case == "B":
            cass = not cass
        if any(np.all(next_pos == a) for a in teleporters):
            if np.all(next_pos == teleporter[0]):
                pos = teleporters[1]
            else:
                pos = teleporters[0]
            yield cur_dir
            print("Debug messages... teleporting", file=sys.stderr, flush=True)
            continue
        if next_case == "#" or (next_case == "X" and not cass):
            print("Debug messages... obstacle", dir_idx, file=sys.stderr, flush=True)
            cur_dir = priorities[dir_idx]
            dir_idx += 1
            continue

        pos = pos + depl[cur_dir]
        path.append(pos)
        dir_idx = 0
        yield cur_dir


l, c = [int(i) for i in input().split()]
map_ = []
for _ in range(l):
    map_.append(list(input()))

print("Debug messages... map\n", np.array(map_), file=sys.stderr, flush=True)

teleporter = which(map_, "T")
teleporters = []
if teleporter is not None:
    teleporters = [np.array(teleporter)] * 2
    print("Debug messages... t", teleporter, file=sys.stderr, flush=True)
    print("Debug messages... t", teleporters, file=sys.stderr, flush=True)
    map_[teleporter[0]][teleporter[1]] = "t"
    teleporters[1] = which(map_, "T")
    teleporters.append(teleporter)
    print("Debug messages... t", teleporters, file=sys.stderr, flush=True)

init_pos = which(map_, "@")


for step in steps(map_, init_pos, teleporters):
    print("----", step, file=sys.stderr, flush=True)
    print(step)
