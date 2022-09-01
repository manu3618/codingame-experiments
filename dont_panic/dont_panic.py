# https://www.codingame.com/ide/puzzle/don't-panic
(_, _, _, exit_f, exit_p, _, _, nb_e) = [int(i) for i in input().split()]

e = {exit_f: exit_p}
for i in range(nb_e):
    e_f, e_p = [int(j) for j in input().split()]
    e[e_f] = e_p

while True:
    i = input().split()
    c_f = int(i[0])
    c_p = int(i[1])
    d = i[2]

    if (d == "LEFT" and e[c_f] > c_p) or (d == "RIGHT" and e[c_f] < c_p):
        print("BLOCK")
    else:
        print("WAIT")
