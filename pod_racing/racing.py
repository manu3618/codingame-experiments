# https://www.codingame.com/ide/puzzle/mad-pod-racing
import sys


thrust = 100
boost_used = False
old_dist = 0
# game loop
while True:
    # next_checkpoint_x: x position of the next check point
    # next_checkpoint_y: y position of the next check point
    # next_checkpoint_dist: distance to the next checkpoint
    # next_checkpoint_angle: angle between your pod orientation and the direction of the next checkpoint
    (
        x,
        y,
        next_checkpoint_x,
        next_checkpoint_y,
        next_checkpoint_dist,
        next_checkpoint_angle,
    ) = [int(i) for i in input().split()]
    opponent_x, opponent_y = [int(i) for i in input().split()]
    print(
        "Debug messages...",
        next_checkpoint_dist,
        next_checkpoint_angle,
        file=sys.stderr,
        flush=True,
    )

    thrusts = (
        1.3 * (150 - abs(next_checkpoint_angle)),
        next_checkpoint_dist * (150 - abs(next_checkpoint_angle)) / 10000,
        0,
    )
    print("Debug messages...", thrusts, file=sys.stderr, flush=True)

    thrust = min(100, max(thrusts))
    thrust = int(thrust)

    if next_checkpoint_dist < 300 and abs(next_checkpoint_angle) > 90:
        pass
        thrust = 0

    if next_checkpoint_dist > 2500 and abs(next_checkpoint_angle) < 15:
        pass
        thrust = 100

    if abs(next_checkpoint_angle) < 5:
        pass
        # thrust = 100
    if abs(old_dist - next_checkpoint_dist) < 1:
        thrust = 10

    # You have to output the target position
    # followed by the power (0 <= thrust <= 100)
    # i.e.: "x y thrust"
    if (
        thrust == 100
        and not boost_used
        and next_checkpoint_dist > 10000
        and abs(next_checkpoint_angle) < 5
    ):
        print(str(next_checkpoint_x) + " " + str(next_checkpoint_y) + " BOOST")
        boost_used = True
    else:
        print(str(next_checkpoint_x) + " " + str(next_checkpoint_y) + " " + str(thrust))
