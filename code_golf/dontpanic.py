#https://www.codingame.com/ide/puzzle/don't-panic
c=lambda:input().split()
g=int
*_,f,p,_,_,b=c()
e={f:p}
exec("f,p=c();e[f]=p;"*g(b))
while 1:f,p,d=c();p=g(p);print(["WAIT","BLOCK"][(d[0]=="L"and g(e[f])>p)|(d[0]=="R"and g(e[f])<p)])
