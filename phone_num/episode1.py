# https://www.codingame.com/ide/puzzle/telephone-numbers
import math
import sys

phone_nums = set()

n = int(input())
storage = 0


def get_max_common_len(a: str, b: str):
    for idx in range(len(a) + 1, 0, -1):
        if a[:idx] == b[:idx]:
            return idx
    return 0


def get_additional_storage(num: str, phone_numbers):
    for idx in range(len(num) + 1, 0, -1):
        for phone in phone_numbers:
            if num[:idx] == phone[:idx]:
                return len(num) - idx
    return len(num)


for _ in range(n):
    telephone = input()
    storage += get_additional_storage(telephone, phone_nums)
    phone_nums.add(telephone)


print(storage)
