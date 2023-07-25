# https://www.codingame.com/ide/puzzle/death-first-search-episode-1
import sys
from collections import defaultdict

graph = defaultdict(list)
# n: the total number of nodes in the level, including the gateways
# l: the number of links
# e: the number of exit gateways
n, l, e = [int(i) for i in input().split()]
gateways = []


def get_next_step(source):
    max_cost = 50
    if source in gateways:
        return
    for g in gateways:
        if g in graph[source]:
            return g
    reachable = {n: set(graph[n]) for n in graph[source]}
    print("Debug messages 18", source, reachable, file=sys.stderr, flush=True)
    for cost in range(max_cost):
        for n in graph[source]:
            if any(g in reachable[n] for g in gateways):
                print("Debug messages 22", n, gateways, file=sys.stderr, flush=True)
                return n
            new_dest = []
            for r in reachable[n]:
                new_dest.extend(graph[r])
            reachable[n].update(set(new_dest))
            print("Debug messages 24", cost, n, reachable, file=sys.stderr, flush=True)


def generate_path(source):
    next_step = source
    max_cost = 50
    for _ in range(max_cost):
        next_step = get_next_step(next_step)
        print("Debug messages 32", source, next_step, file=sys.stderr, flush=True)
        if next_step is None or next_step == source:
            return
        yield next_step


def get_path(source):
    p = [s for s in generate_path(source)]
    p.insert(0, source)
    print("Debug messages 44", p, file=sys.stderr, flush=True)
    if p[-1] in gateways:
        return p
    for g in gateways:
        if g in graph[p[-1]]:
            p.append(g)
            return p
    return p


# initialization
for i in range(l):
    # n1: N1 and N2 defines a link between these nodes
    n1, n2 = [int(j) for j in input().split()]
    graph[n1].append(n2)
    graph[n2].append(n1)
for i in range(e):
    gateways.append(int(input()))


# game loop
while True:
    si = int(
        input()
    )  # The index of the node on which the Bobnet agent is positioned this turn
    p = get_path(si)
    print("Debug messages...", si, p, gateways, file=sys.stderr, flush=True)
    print("Debug messages...", graph, file=sys.stderr, flush=True)
    leg = p[0], p[1]
    # leg = p[-2], p[-1]
    print(*leg)
    graph[leg[0]].remove(leg[1])
    graph[leg[1]].remove(leg[0])
