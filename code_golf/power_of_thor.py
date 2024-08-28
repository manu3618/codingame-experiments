#https://www.codingame.com/ide/puzzle/power-of-thor
i=input
x,y,X,Y=map(int,i().split())
while 1:
 i();d=''
 if Y<y:d+="S";Y+=1
 if Y>y:d+="N";Y-=1
 if X<x:d+="E";X+=1
 if X>x:d+="W";X-=1
 print(d
