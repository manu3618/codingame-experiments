# https://www.codingame.com/ide/puzzle/chuck-norris-codesize
from itertools import groupby as g
print(' '.join(("0 "if a=="1"else"00 ")+"0"*len(list(b))for a,b in g(''.join(f"{ord(c):#010b}"[3:]for c in input()))))
