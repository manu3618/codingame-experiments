import sys
import math

b = int(input())
n = input()


def is_palindrome(n: str) -> bool:
    return n == n[::-1]


def is_prime(n: int) -> bool:
    if n <= 1:
        return False
    if not int(n) == n:
        # not an integer
        return False

    for d in range(2, n // 2):
        if n % d == 0:
            return False

    return True


def to_base(x: int, base=10) -> str:
    """return representation of x in base base"""
    sign = "-" if x < 0 else ""
    rep = ""
    x = abs(x)
    while x != 0:
        rep = str(x % base) + rep
        x = x // base
    return sign + rep


solution = max(
    x for x in range(int(n, base=b)) if is_palindrome(to_base(x, b)) and is_prime(x)
)

print(to_base(solution, b))
