# https://www.codingame.com/ide/puzzle/power-of-thor-episode-1

# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.
# ---
# Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.

# light_x: the X position of the light of power
# light_y: the Y position of the light of power
# initial_tx: Thor's starting X position
# initial_ty: Thor's starting Y position
light_x, light_y, initial_tx, initial_ty = [int(i) for i in input().split()]

cur_x, cur_y = initial_tx, initial_ty

# game loop
while True:
    remaining_turns = int(
        input()
    )  # The remaining amount of turns Thor can move. Do not remove this line.

    # Write an action using print
    # To debug: print("Debug messages...", file=sys.stderr, flush=True)

    direction = ""
    if cur_y > light_y:
        direction += "N"
        cur_y += -1

    if cur_y < light_y:
        direction += "S"
        cur_y += 1

    if cur_x > light_x:
        direction += "W"
        cur_x += -1

    if cur_x < light_x:
        direction += "E"
        cur_x += 1

    print(direction)
