def hamming(a, b):
    counter = 0
    for n in range(len(a)):
        if a[n] != b[n]:
            counter += 1
    return counter
