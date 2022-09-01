_,_,_,f,p,_,_,b=[int(i) for i in input().split()]
e={f:p}
for i in range(b):
    f,p=[int(j) for j in input().split()]
    e[f]=p
while 1:
    f,p,d=input().split()
    f=int(f)
    p=int(p)
    if (d=="LEFT" and e[f]>p)or(d=="RIGHT" and e[f]<p):
        print("BLOCK")
    else:
        print("WAIT")
