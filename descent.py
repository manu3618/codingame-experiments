# https://www.codingame.com/ide/puzzle/the-descent

import sys
import math

# The while loop represents the game.
# Each iteration represents a turn of the game
# where you are given inputs (the heights of the mountains)
# and where you have to print an output (the index of the mountain to fire on)
# The inputs you are given are automatically updated according to your last actions.


# game loop
while True:
    mountains= []
    for i in range(8):
        mountains.append(int(input()))  # represents the height of one mountain.

    # Write an action using print
    # To debug: print("Debug messages...", file=sys.stderr, flush=True)

    # The index of the mountain to fire on.
    heights = list(enumerate(mountains))
    heights.sort(key=lambda x: x[-1])

    print("Debug messages..." , heights, file=sys.stderr, flush=True)
    print(heights[-1][0])
