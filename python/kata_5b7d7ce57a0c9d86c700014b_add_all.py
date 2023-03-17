# from itertools import combinations, permutations
import heapq


# def add_all(lst):
#     lc = list()
#     for n in range(len(lst) + 1):
#         lc += list(combinations(lst, n))
#
#     combo = list(filter(lambda x: len(x) == 2, lc))
#     # print(combo)
#     final = []
#     for x in combo:
#         print(f"x is: {x}")
#         diff = [i for i in list(x) + lst if i not in x or i not in lst]
#         print(f"diff is: {diff}")
#         perm = list(permutations(diff))
#         print(f"perm: {perm}")
#         for p in perm:
#             print(f"p is: {p}")
#             cost = sum(x)
#             print(f"cost is: {cost}")
#             res = []
#             res.append(cost)
#             print(f"res is: {res}")
#             for d in p:
#                 print(f"d: {d}")
#                 curr_cost = cost
#                 cost += d
#                 print(f"d + {curr_cost} = {cost}")
#                 res.append(cost)
#                 # print(f"cost is: {cost}")
#                 print(f"res is: {res}")
#             print(f"current res: {sum(res)}\n")
#             final.append(sum(res))
#     print(f"final is: {final}")
#     print(f"min(final): {min(final)}")
#     return min(final)


# heap queue algorithm
# more info:
#     https://docs.python.org/3/library/heapq.html
#     https://www.geeksforgeeks.org/connect-n-ropes-minimum-cost/
def add_all(lst):
    heapq.heapify(lst)
    res = 0
    while len(lst) > 1:
        first = heapq.heappop(lst)
        second = heapq.heappop(lst)
        res += first + second
        heapq.heappush(lst, first + second)
    print(f"res: {res}")
    return res


def main():
    # l = [1, 2, 3, 4, 5]
    l = [1, 2, 3, 4, 5]
    add_all(l)


if __name__ == "__main__":
    main()
