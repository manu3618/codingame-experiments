import math
import random
import sys
from pprint import pformat
from time import time

import numpy as np
import pandas as pd
from scipy import linalg

START = time()

ENTITIES = {"SHIP": [], "MINE": [], "CANNONBALL": [], "BARREL": []}
PATHS = {}


def debug(*args):
    now = time()
    print(now - START, file=sys.stderr)
    print(pformat(*args), file=sys.stderr, flush=True)


class CaribbeanException(Exception):
    pass


class TargetShortage(CaribbeanException):
    pass


class IllegalMovementError(CaribbeanException):
    pass


class Entity:
    def __init__(self, entity_id, entity_type, x, y, *args, **kwargs):
        self.id = entity_id
        self.type = entity_type
        self.coords = (int(x), int(y))
        self.owner = 0  # other
        if self.type == "SHIP":
            self.direction = int(args[0])
            self.speed = int(args[1])
            self.rhum = int(args[2])
            self.owner = args[3]
        if self.type == "BARREL":
            self.rhum = int(args[0])
        if self.type == "CANNONBALL":
            self.source = args[0]
            self.impact = int(args[1])

    @property
    def cube_coords(self):
        return oddr_to_cube(*self.coords)

    def distance(self, entity):
        return linalg.norm(np.array(entity.cube_coords) - self.cube_coords)

    def __repr__(self):
        msg = "<Entity {} ({})> ({})".format(self.type, self.id, self.coords)
        return msg

    def to_dict(self):
        keys = (
            "id",
            "type",
            "owner",
            "coords",
            "direction",
            "speed",
            "impact",
            "rhum",
            "source",
        )
        return {k: getattr(self, k) for k in keys if hasattr(self, k)}

    def decide(self, game=None):
        """decide what to do"""
        if random.random() < .7:
            try:
                debug("barrels")
                self.go_to_nearest_barrel(game)
                return
            except CaribbeanException:
                debug("no more barrels")
        try:
            debug("mine")
            self.opportunity_mine(game)
            return
        except CaribbeanException:
            debug("no opportunity to mine")

        if random.random() < .1:
            debug("random move")
            goal = random.choice(list(get_neighbors_oddr(*self.coords).values()))
            cmd_move(*goal)
            return

        try:
            debug("fire")
            self.fire(game)
            return
        except CaribbeanException:
            pass
        debug("random move")
        goal = random.choice(list(get_neighbors_oddr(*self.coords).values()))
        cmd_move(*goal)

    def go_to(self, goal, forbidden=None):
        """got to coords by trying to avoid forbidden coords.

        Args:
            coords (tuple) : (x, y) oddr coordinate to go to
            forbiddent (list) : list of forbiddent coordinates
        """
        if forbidden is None:
            forbidden = []
        debug({"goal": goal,  "source": self.coords, "forbidden": forbidden})
        # if goal in get_neighbors_oddr(*self.coords).values():
        #     debug("GOAL in neighbors")
        #     cmd_move(*goal)
        #     return

        # try straight line
        direction = get_direction(self.coords, goal)
        steps = get_next_steps_oddr(self.coords, direction, 1)
        if not any(step in forbidden for step in steps):
            debug(f"straight move (dircetion {direction})")
            cmd_move(*goal)
            # cmd_move(*steps[-1])
            return

        # next 3rd step in more elaborated path_finding
        debug("finding path")
        path = find_path_recurs(self.coords, goal, forbidden)
        idx = min(2, len(path))
        debug({"path": path, "idx": idx})
        cmd_move(*path[idx-1])

    def go_to_nearest_barrel(self, game: pd.DataFrame):
        """print command to go the the nearest barrel"""
        barrel_coords = [bar.coords for bar in ENTITIES["BARREL"]]
        debug({"barrels": barrel_coords})
        cannonballs = ENTITIES["CANNONBALL"]
        mines_ = set(mine.coords for mine in ENTITIES["MINE"])
        mines = {m for mine in mines_ for m in get_neighbors_oddr(*mine).values()}
        mines = mines.union({ball.coords for ball in cannonballs if ball.impact < 3})
        ships = set(ship.coords for ship in ENTITIES["SHIP"])
        debug("forbidden found")
        nearest = oddr_find_nearest(self.coords, barrel_coords)
        debug({"nearest": nearest})
        if not nearest:
            raise TargetShortage
        self.go_to(nearest, mines.union(ships))

    def fire(self, game: pd.DataFrame):
        """Fire where the nearest ship will be."""
        if [b for b in ENTITIES["CANNONBALL"] if b.owner == self.id]:
            # own ball already in the air
            raise IllegalMovementError

        debug("fire")
        debug({"ships": ENTITIES["SHIP"]})
        ships_coords = [s.coords for s in ENTITIES["SHIP"] if s.owner == "0"]
        debug({"coords": ships_coords})
        nearest_coords = oddr_find_nearest(self.coords, ships_coords)
        debug({"nearest": nearest_coords})
        if not nearest_coords:
            raise TargetShortage
        nearest_enemy = next(s for s in ENTITIES["SHIP"] if s.coords == nearest_coords)

        trajectory = get_next_steps_oddr(
            nearest_enemy.coords, nearest_enemy.direction, nearest_enemy.speed
        )
        bow = get_neighbors_oddr(*self.coords)[self.direction]
        for turn, position in enumerate(trajectory):
            time = round(1 + oddr_distance(bow, position) / 3)
            if time <= turn + 1:
                cmd_fire(*position)
                return
        raise TargetShortage

    def opportunity_mine(self, game: pd.DataFrame):
        """drop a mine if there is a chance it hurts enemy ship."""
        try:
            nearest_enemy, distance = min(
                (
                    (s, oddr_distance(s.coords, self.coords))
                    for s in ENTITIES["SHIP"]
                    if s.owner == 0
                ),
                key=lambda x: x[1],
            )
        except ValueError:
            raise TargetShortage

        if not self.is_in_front(nearest_enemy) and distance < 3:
            print("MINE")
            return
        raise TargetShortage

    def is_in_front(self, coords):
        """Return True if the (oodr) coordinates are in the fron sector"""
        direction = get_direction(self.coords, coords)
        front_dirs = [(self.direction + i) % 6 for i in range(-1, 1)]
        return direction in front_dirs

    def opportunity_fire(self, boat):
        if self.is_in_front(boat.coords):
            # fire
            pass
        else:
            # mine
            pass
        raise NotImplementedError


def cube_to_oddr(x, y, z):
    return (x + (z - z % 2) / 2, z)


def oddr_to_cube(col, row):
    x = col - (row - row % 2) / 2
    return x, -x - row, row


def oddr_linalg_distance(a, b):
    """distance betweeen point a and b"""
    return linalg.norm(np.array(oddr_to_cube(*a)) - oddr_to_cube(*b))


def oddr_distance(a, b):
    """distance between a and b"""
    neighbors = {a}
    for distance in range(30):
        if b in neighbors:
            return distance
        neighbors = set.union(
            *[set(get_neighbors_oddr(*d).values()) for d in neighbors]
        )
    return float("inf")


def oddr_find_nearest(src, dst):
    """Find nearest coords among destinations

    Args:
        src (tuple) : source coordinates
        dst (list): iterable of coordinates
    Returns:
        (tuple) one set of coordinates among dst
    """
    if not dst:
        return dst
    neighbors = {src}
    for _ in range(30):
        for coords in dst:
            if coords in neighbors:
                return coords
        neighbors = set.union(
            *[set(get_neighbors_oddr(*d).values()) for d in neighbors]
        )
    return dst[0]


def cmd_move(x, y):
    print(" ".join(("MOVE", str(int(x)), str(int(y)))))


def cmd_fire(x, y):
    print(" ".join(("FIRE", str(int(x)), str(int(y)))))


def find_path(start, end, forbidden=None, max_len=10):
    """quick and dirty A*

    Args:
        start (tuple): start coordinates in oddr
        end (tuple): end coordinates in oddr
        forbidden (iterable): list of forbidden coordinates
    Returns:
        (tuple): next step
    """
    if end == start or end in get_neighbors_oddr(*start).items():
        return end

    if forbidden is None:
        forbidden = set()

    # {first_node: {visitable_node: cost}}
    first = {node: {node: 1} for node in get_neighbors_oddr(*start).values()}
    for _ in range(max_len):
        visited = {k: v for node in first.values() for k, v in node.items()}
        cur_first = first.copy()
        for init, nodes in cur_first.items():
            if end in nodes:
                return init
            cur_nodes = nodes.copy()
            for node, cost in cur_nodes.items():
                for next_node in get_neighbors_oddr(*node).values():
                    if next_node in forbidden or next_node in visited:
                        continue
                    first[init][next_node] = cost + 1
    return end


def find_path_recurs(start, end, forbidden=None, max_len=10):
    """find path recusvely

    Returns:
        (list) list of coordinates from start to end
    """
    if (start, end) in PATHS:
        return PATHS[(start, end)]
    if (end, start) in PATHS:
        return PATHS[(end, start)][::-1]
    if max_len < 1:
        raise StopIteration
    if forbidden is None:
        forbidden = []
    forbidden = list(forbidden)
    if start == end:
        return []

    forbidden.append(start)

    neighbors = [
        node for node in get_neighbors_oddr(*start).values() if node not in forbidden
    ]
    candidates = []
    for n in neighbors:
        try:
            candidates.append([n,  *find_path_recurs(n, end, forbidden, max_len - 1)])
        except StopIteration:
            continue
    if not candidates:
        # return too long path
        return [(end)] * 100

    min_path, _ = min([(c, len(c)) for c in candidates], key=lambda x: x[1])
    PATHS[(start, end)] = min_path
    return min_path


def iter_path(start, end, forbidden=None, max_len=100):
    iterator = path_iterator(start, end, forbidden, max_len)
    for step in iterator:
        yield step


def path_iterator(start, end, forbidden=None, max_len=100):
    """iterator over path steps"""
    current = start
    for _ in range(max_len):
        if current == end:
            return end
        current = find_path(current, end, forbidden, max_len)
        yield current


def get_neighbors_oddr(x, y):
    """return neighbors of the case in oddr coordinates.

    Retruns:
        (dict) {direction: neighbor}
    """
    if y % 2 == 0:
        # odd line
        return {
            2: (x - 1, y - 1),
            1: (x, y - 1),
            3: (x - 1, y),
            0: (x + 1, y),
            4: (x - 1, y + 1),
            5: (x, y + 1),
        }
    else:
        # even line
        return {
            2: (x, y - 1),
            1: (x + 1, y - 1),
            3: (x - 1, y),
            0: (x + 1, y),
            4: (x, y + 1),
            5: (x + 1, y + 1),
        }


def get_direction(start, end):
    """Return best direction from start to end"""
    min_dist = min(
        ((d, oddr_distance(end, v)) for d, v in get_neighbors_oddr(*start).items()),
        key=lambda x: x[1],
    )
    return min_dist[0]


def get_next_steps_oddr(coords, direction, speed=1, number=5):
    """Get next steps if the object continue its movement "steady as she goes".

    Args:
        coords (tuple): oddr coords
        direction (int)
        speed (int)
        number (int) : number of steps
    Returns:
        (list) coords
    """
    ret = []
    for steps in range(number):
        for _ in range(speed):
            coords = get_neighbors_oddr(*coords)[direction]
        ret.append(coords)
    return ret


if __name__ == "__main__":
    # game loop
    while True:
        START = time()
        my_ship_count = int(input())  # the number of remaining ships

        # the number of entities (e.g. ships, mines or cannonballs)
        entity_count = int(input())
        # reset globals
        keys = tuple(ENTITIES.keys())
        for k in keys:
            ENTITIES[k] = []
        PATHS = {}
        debug(PATHS)

        # fill in entities
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
            ENTITIES[entity_type].append(entities[entity_id])

        # debug(ENTITIES)
        # entities_dict = {k: v.to_dict() for k, v in entities.items()}
        # entities_df = pd.DataFrame().from_dict(entities_dict, orient="index")
        # debug(entities_df)

        for i in range(my_ship_count):
            # Write an action using print
            # To debug: print("Debug messages...", file=sys.stderr, flush=True)

            # Any valid action, such as "WAIT" or "MOVE x y"
            # debug(i, my_ship_count)
            my_ship = entities[i]
            my_ship.decide()
            # my_ship.go_to_nearest_barrel(entities_df)
