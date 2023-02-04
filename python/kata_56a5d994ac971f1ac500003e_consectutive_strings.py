from itertools import tee, islice, chain, repeat


def pairwise(itt):
    a, b = tee(itt)
    next(b, None)
    return zip(a, b)


def longest_consec(strarr, k):
    for _ in range(0, len(strarr), k):
        print(strarr[::k])


def chunk_pad(it, size, padval=None):
    it = chain(iter(it), repeat(padval))
    return iter(lambda: tuple(islice(it, size)), (padval,) * size)


def main():
    l = ["zone", "abigail", "theta", "form", "libe", "zas"]
    ll = list(chunk_pad(l, 3))
    # longest_consec(l, 2)
    print("pairwise\n")
    print(ll)
    # for x, y in pairwise(l):
    #     print(x, y)


if __name__ == "__main__":
    main()
