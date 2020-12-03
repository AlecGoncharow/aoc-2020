from collections import *
from functools import lru_cache
import heapq
import itertools
import math
import random
import sys

def main():
    fin = open("input.txt", "r")
    lines = [line.strip() for line in fin.readlines() if line.strip()]
    p1, p2 = 0, 0
    left_x = 0

    ans = []

    patterns = [1, 3, 5, 7, 1]

    for (i, line) in enumerate(lines[1:]):
        left_x += 3
        if len(line) <= left_x:
            left_x %= len(line)
        if line[left_x] == '#':
            p1 += 1

    for (j, pattern) in enumerate(patterns):
        p2 = 0
        left_x = 0
        for (i, line) in enumerate(lines[1:]):
            if j == 4 and (i + 1) % 2:
                continue

            left_x += pattern
            if len(line) <= left_x:
                left_x %= len(line)
            if line[left_x] == '#':
                p2 += 1


        ans.append(p2)


    print(p1)
    out = 1
    for a in ans:
        out *= a
    print(out)

if __name__ == "__main__":
    main()
