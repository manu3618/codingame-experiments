# https://www.codingame.com/ide/puzzle/temperature-code-golf
input()
p=print
s=sorted(map(int,input().split()),key=lambda k:-abs(k))
if len(s)==0:s=[0]
a=s[-1]
if a in s and -a in s:p(abs(a))
else:p(a)
