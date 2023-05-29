# https://www.codingame.com/ide/puzzle/mayan-calculation


def to_base_n(x: int, n=20):
    res = []
    while x:
        res.append(x % n)
        x = x // n
    return res[::-1]


l, h = [int(i) for i in input().split()]
all_nums = []
maya_to_dec = {}
dec_to_maya = {}
for _ in range(h):
    all_nums.append(input())

for n in range(20):
    representation = "\n".join([all_nums[i][(l * n) : l * (n + 1)] for i in range(h)])
    maya_to_dec[representation] = n
    dec_to_maya[n] = representation

s1 = int(input())
num1 = ["\n".join(input() for _ in range(h)) for _ in range(s1 // h)]
num1 = [maya_to_dec[c] for c in num1]
num1 = sum([num1[-i - 1] * 20**i for i in range(len(num1))])

s2 = int(input())
num2 = ["\n".join(input() for _ in range(h)) for _ in range(s2 // h)]
num2 = [maya_to_dec[c] for c in num2]
num2 = sum([num2[-i - 1] * 20**i for i in range(len(num2))])

operation = input()

result = eval(f"{num1} {operation} {num2}")


result = to_base_n(result)


if len(result) == 0:
    print(dec_to_maya[0])
print("\n".join(dec_to_maya[c] for c in result))
