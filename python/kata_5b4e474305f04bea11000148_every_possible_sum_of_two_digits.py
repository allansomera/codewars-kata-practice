from itertools import combinations


def digits(sums):
    s = str(sums)
    l = [int(x) for x in s]
    combo = list()
    lc = list()
    for n in range(len(l) + 1):
        lc += list(combinations(l, n))
    combo = list(filter(lambda x: len(x) == 2, lc))
    print(f"combo: {combo}")
    res = [sum(x) for x in combo]
    print(res)


def main():
    digits(156)


if __name__ == "__main__":
    main()

# solutions:
# from itertools import combinations
#
# def digits(num):
#     return list(map(sum, combinations(map(int,str(num)),2)))
#
#
# def digits(num):
#     return [int(a) + int(b) for a, b in combinations(str(num), 2)]
#
#
# def digits(num):
#     return [sum(pair) for pair in combinations([int(d) for d in str(num)], 2)]
