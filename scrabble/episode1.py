# https://www.codingame.com/ide/puzzle/scrabble
import sys
from collections import Counter
from itertools import permutations

# points
pts = [
    (("eaionrtlsu"), 1),
    (("dg"), 2),
    (("bcmp"), 3),
    (("fhvwy"), 4),
    (("k"), 5),
    (("jx"), 6),
    (("qz"), 10),
]
points = {}
for letters, point in pts:
    for c in letters:
        points[c] = point


# dictionnary
n = int(input())
words = []
for i in range(n):
    words.append(input())

# letters
letters = list(input())


# solver
valid_words = Counter()
attempts = (
    w
    for k in range(1, len(letters) + 1)
    for le in permutations(letters, k)
    if (w := "".join(le)) in words
)

for attempt in attempts:
    valid_words[attempt] = sum(points[c] for c in attempt)

max_score = valid_words.most_common(1)[0][1]
answers = [k for k, v in valid_words.items() if v == max_score]

print(next(w for w in words if w in answers))
