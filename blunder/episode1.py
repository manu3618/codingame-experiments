# https://www.codingame.com/ide/puzzle/blunder-episode-1
import sys

import numpy as np

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.


def which(arr, c="@"):
    for idx, row in enumerate(arr):
        if c in row:
            return idx, list(row).index(c)


def has_loop(path):
    if len(path) < 1000:
        return False

    path = [tuple(elt) for elt in path]
    le = len(path) // 2
    for i in range(le - 1):
        if path[i : i + le] == path[-le:]:
            print("Debug messages... loop", file=sys.stderr, flush=True)
            print("path", path, file=sys.stderr, flush=True)
            print(f"{i}", path[i : i + le], file=sys.stderr, flush=True)
            print(". ", path[-le:], file=sys.stderr, flush=True)
            return True
    return False


def steps(arr, pos, teleporters=()):
    pos = np.array(pos)
    priorities = ["SOUTH", "EAST", "NORTH", "WEST"]
    depl = {"SOUTH": (1, 0), "EAST": (0, 1), "NORTH": (-1, 0), "WEST": (0, -1)}
    cur_dir = "SOUTH"
    dirs = {w[0]: w for w in priorities}
    cass = False
    dir_idx = 0
    path = []
    beer_taken = False
    teleporter_taken = False
    while True:
        next_pos = pos + depl[cur_dir]
        next_case = arr[next_pos[0]][next_pos[1]]
        cur_case = arr[pos[0]][pos[1]]
        print(
            "Debug messages... begining",
            pos,
            next_pos,
            depl[cur_dir],
            cur_case,
            next_case,
            file=sys.stderr,
            flush=True,
        )
        if has_loop(path[:-1]):
            print("Debug messages... loop", path, file=sys.stderr, flush=True)
            yield "LOOP"
            return
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
            print("Debug messages... breaking obstacle", file=sys.stderr, flush=True)
            path = []
            continue
        if cur_case == "B" and not beer_taken:
            cass = not cass
            print("Debug messages... beer", cass, file=sys.stderr, flush=True)
            beer_taken = True
        if (tuple(next_pos) in teleporters) and not teleporter_taken:
            print("Debug messages... pos", pos, file=sys.stderr, flush=True)
            idx = teleporters.index(tuple(next_pos))
            pos = np.array(teleporters[(idx + 1) % 2])
            print("Debug messages... idx", idx, file=sys.stderr, flush=True)
            print("Debug messages... pos", pos, file=sys.stderr, flush=True)
            yield cur_dir
            print("Debug messages... teleporting", file=sys.stderr, flush=True)
            teleporter_taken = True
            continue
        if next_case == "#" or (next_case == "X" and not cass):
            print("Debug messages... obstacle", dir_idx, file=sys.stderr, flush=True)
            cur_dir = priorities[dir_idx]
            dir_idx += 1
            continue

        pos = pos + depl[cur_dir]
        path.append(pos)
        dir_idx = 0
        beer_taken = False
        teleporter_taken = False
        yield cur_dir


l, c = [int(i) for i in input().split()]
map_ = []
for _ in range(l):
    map_.append(list(input()))

# print("Debug messages... map\n", np.array(map_), file=sys.stderr, flush=True)

teleporter = which(map_, "T")
# print("Debug messages... map\n", np.array(map_), file=sys.stderr, flush=True)
for idx, row in enumerate(map_):
    print(f"#{idx}\t", row, file=sys.stderr, flush=True)

teleporters = []
if teleporter is not None:
    teleporters = [np.array(teleporter)] * 2
    map_[teleporter[0]][teleporter[1]] = "t"
    teleporters[1] = which(map_, "T")
    teleporters.append(teleporter)
    teleporters = [tuple(elt) for elt in teleporters]

print("Debug messages... init pos", file=sys.stderr, flush=True)
init_pos = which(map_, "@")


print("Debug messages... GO!", file=sys.stderr, flush=True)
path = list(steps(map_, init_pos, teleporters))

if path[-1] == "LOOP":
    print("LOOP")
else:
    print("\n".join(path))

# for idx, row in enumerate(map_):
#     print(f"#{idx}\t", row, file=sys.stderr, flush=True)
