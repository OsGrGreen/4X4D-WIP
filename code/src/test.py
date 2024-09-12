results = []
N = 1
q0 = 10
r0 = 21
for q in range(-N,N+1):
    print("Q:" , q)
    r1 = max(-N, -q - N)
    r2 = min(N, -q + N)
    for r in range(r1,r2+1,1):
        print("R:" ,r)
        s = -q-r
        results.append((q0+q,r0+r))

print(results)