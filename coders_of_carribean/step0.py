import math
import sys
from pprint import pformat

import numpy as np
from scipy import linalg


def debug(*args):
    print(pformat(*args), file=sys.stderr, flush=True)


class Entity:
    def __init__(self, entity_id, entity_type, x, y, *args, **kwargs):
        self.id = entity_id
        self.type = entity_type
        self.coords = (int(x), int(y))

    @property
    def cube_coords(self):
        return oddr_to_cube(*self.coords)

    def distance(self, entity):
        return linalg.norm(np.array(entity.cube_coords) - self.cube_coords)

    def direction(self, entity):
        pass

    def __repr__(self):
        msg = "<Entity {} ({})> ({})".format(self.type, self.id, self.coords)
        return msg


def cube_to_oddr(x, y, z):

    return (x + (z - z % 2) / 2, z)


def oddr_to_cube(col, row):
    x = col - (row - row % 2) / 2
    return x, -x - row, row


def cmd_move(x, y):
    print(" ".join(("MOVE", str(int(x)), str(int(y)))))


if __name__ == "__main__":
    # game loop
    while True:
        my_ship_count = int(input())  # the number of remaining ships

        # the number of entities (e.g. ships, mines or cannonballs)
        entity_count = int(input())
        entities = {}
        for i in range(entity_count):
            inputs = input().split()
            entity_id = int(inputs[0])
            entity_type = inputs[1]
            x = int(inputs[2])
            y = int(inputs[3])
            arg_1 = int(inputs[4])
            arg_2 = int(inputs[5])
            arg_3 = int(inputs[6])
            arg_4 = int(inputs[7])
            entities[entity_id] = Entity(*inputs)

        for i in range(my_ship_count):
            # Write an action using print
            # To debug: print("Debug messages...", file=sys.stderr, flush=True)

            # Any valid action, such as "WAIT" or "MOVE x y"
            debug(i, my_ship_count)
            # debug(entities)
            my_ship = entities[i]

            barrels = {k: v for k, v in entities.items() if v.type == "BARREL"}
            debug(barrels)

            debug([(k, my_ship.distance(v)) for k, v in barrels.items()])
            nearest_barrel = min(
                ((k, my_ship.distance(v)) for k, v in barrels.items()),
                key=lambda x: x[1],
            )
            cmd_move(*barrels[nearest_barrel[0]].coords)
            # print("MOVE 11 10")
