
lol = []
for n in range(0,3**6):
    a = n % 3
    n //= 3
    b = n % 3
    n //= 3
    c = n % 3
    n //= 3
    d = n % 3
    n //= 3
    e = n % 3
    n //= 3
    f = n % 3
    n //= 3
    sum = (f << 5*2 )+( e << 4*2) + (d << 3*2) + (c << 2*2) + (b << 2) +( a << 0)
    lol.append(sum)

plol = "const F3_INDEX: u16 = ["
for a in lol:
    plol += str(a) + ", "

plol += "];"
print(plol)