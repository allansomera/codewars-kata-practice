from itertools import tee, islice, chain, repeat, pairwise
from more_itertools import sliding_window


# def pairwise(itt):
#     a, b = tee(itt)
#     next(b, None)
#     return zip(a, b)


def longest_consec(strarr, k):
    return (
        ""
        if len(strarr) == 0 or k <= 0 or k > len(strarr)
        else sorted(
            ["".join(x) for x in sliding_window(strarr, k)], key=len, reverse=True
        )[0]
    )


# def chunk_pad(it, size, padval=""):
#     it = chain(iter(it), repeat(padval))
#     return iter(lambda: tuple(islice(it, size)), (padval,) * size)


def main():
    l = ["zone", "abigail", "theta", "form", "libe", "zas"]
    ll = ["tree", "foling", "trashy", "blue", "abcdef", "uvwxyz"]
    # ll = list(chunk_pad(l, 2))
    # longest_consec(l, 2)
    # lll = pairwise(l)
    # print("pairwise\n")
    # print(lll)
    # for x in ll:
    #     print(x)
    # for x, y in pairwise(l):
    #     print(x, y)

    # print(sorted(res, key=len, reverse=True))
    # print(res)
    # res = sliding_window(l, 2)
    # res = ["".join(x) for x in sliding_window(ll, 2)]
    # print(sorted(res, key=len, reverse=True)[0])
    print(longest_consec(l, 2))


if __name__ == "__main__":
    main()
