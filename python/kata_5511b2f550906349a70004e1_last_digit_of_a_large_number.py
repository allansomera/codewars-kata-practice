def last_digit(n1, n2):
    return pow(int(n1), int(n2), 10)


# src:
#     https://stackoverflow.com/questions/5246856/how-did-python-implement-the-built-in-function-pow
#     https://codereview.stackexchange.com/questions/263312/last-digit-of-any-reasonably-large-ab-analytically/263321#263321
#     https://brilliant.org/wiki/finding-the-last-digit-of-a-power/#:%7E:text=Finding%20the%20last%20digit%20of%20a%20positive%20integer%20is%20the,division%20by%20n%20n%20n.

# solutions:
# digits = {
#     0:[0,0,0,0],
#     1:[1,1,1,1],
#     2:[2,4,8,6],
#     3:[3,9,7,1],
#     4:[4,6,4,6],
#     5:[5,5,5,5],
#     6:[6,6,6,6],
#     7:[7,9,3,1],
#     8:[8,4,2,6],
#     9:[9,1,9,1]
# }
# def last_digit(n1, n2):
#     return digits[n1%10][(n2-1)%4] if n2 else 1
# # --
# def last_digit(n1, n2):
#     last_dict = {
#         0: [0],
#         1: [1],
#         2: [2, 4, 6, 8],
#         3: [3, 9, 7, 1],
#         4: [4, 6],
#         5: [5],
#         6: [6],
#         7: [7, 9, 3, 1],
#         8: [8, 4, 2, 6],
#         9: [9, 1]}
#
#     if n2 == 0:
#         return 1
#     a = n1 % 10
#     return last_dict[a][(n2-1)%len(last_dict[a])]

# def last_digit(n1, n2):
#     return (n1 % 10) ** (n2 % 4 + 4 * bool(n2)) % 10
