from itertools import tee


def pairwise(itt):
    a, b = tee(itt)
    next(b, None)
    return zip(a, b)


def longest_consec(strarr, k):
    for _ in range(0, len(strarr), k):
        print(strarr[::k])


def main():
    l = ["zone", "abigail", "theta", "form", "libe", "zas"]
    longest_consec(l, 2)
    print("pairwise\n")
    for x, y in pairwise(l):
        print(x, y)


if __name__ == "__main__":
    main()
