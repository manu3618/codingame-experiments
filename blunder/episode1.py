# https://www.codingame.com/ide/puzzle/blunder-episode-1

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
    has_inverted = False
    while True:
        next_pos = pos + depl[cur_dir]
        next_case = arr[next_pos[0]][next_pos[1]]
        cur_case = arr[pos[0]][pos[1]]
        if has_loop(path[:-1]):
            yield "LOOP"
            return
        if cur_case == "$":
            return cur_dir
        if cur_case in " @":
            pass
        if cur_case in "SENW":
            if cur_dir != dirs[cur_case]:
                cur_dir = dirs[cur_case]
                continue
            cur_dir = dirs[cur_case]
        if cur_case == "I" and not has_inverted:
            priorities = priorities[::-1]
            has_inverted = True
        if cur_case == "X" and cass:
            arr[pos[0]][pos[1]] = " "
            path = []
            continue
        if cur_case == "B" and not beer_taken:
            cass = not cass
            beer_taken = True
        if (tuple(pos) in teleporters) and not teleporter_taken:
            idx = teleporters.index(tuple(pos))
            pos = np.array(teleporters[(idx + 1) % 2])
            teleporter_taken = True
            continue
        if next_case == "#" or (next_case == "X" and not cass):
            cur_dir = priorities[dir_idx]
            dir_idx += 1
            continue

        pos = pos + depl[cur_dir]
        path.append(pos)
        dir_idx = 0
        beer_taken = False
        teleporter_taken = False
        has_inverted = False
        yield cur_dir


l, c = [int(i) for i in input().split()]
map_ = []
for _ in range(l):
    map_.append(list(input()))

teleporter = which(map_, "T")

teleporters = []
if teleporter is not None:
    teleporters = [np.array(teleporter)] * 2
    map_[teleporter[0]][teleporter[1]] = "t"
    teleporters[1] = which(map_, "T")
    teleporters.append(teleporter)
    teleporters = [tuple(elt) for elt in teleporters]

init_pos = which(map_, "@")


path = list(steps(map_, init_pos, teleporters))


if path[-1] == "LOOP":
    print("LOOP")
else:
    print("\n".join(path))
