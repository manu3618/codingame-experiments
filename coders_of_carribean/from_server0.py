import math
import sys

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.


# game loop
while True:
    my_ship_count = int(input())  # the number of remaining ships
    entity_count = int(
        input()
    )  # the number of entities (e.g. ships, mines or cannonballs)
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
    for i in range(my_ship_count):

        # Write an action using print
        # To debug: print("Debug messages...", file=sys.stderr, flush=True)

        # Any valid action, such as "WAIT" or "MOVE x y"
        print("MOVE 11 10")
