from itertools import permutations


def middle_permutation(string):
    # res = ["".join(x) for x in list(permutations(list(string), len(string)))]
    # res = string.chars.sort.join("")
    # set_res = set(res)
    # s_res = sorted(set_res)
    # s_res = sorted(res)
    #
    # pos = s_res[len(s_res) // 2] if len(s_res) % 2 == 1 else res[(len(s_res) // 2) - 1]
    # print(f"s_res(len): {len(s_res)}")
    # print(f"pos: {pos}")

    # s = sorted(string)
    # if len(s) % 2 == 0:
    #     return s.pop(len(s) // 2 - 1) + "".join(s[::-1])
    # else:
    #     return s.pop(len(s) // 2) + middle_permutation(s)

    s = sorted(string)
    return (
        s.pop(len(s) // 2 - 1) + "".join(s[::-1])
        if len(s) % 2 == 0
        else s.pop(len(s) // 2) + middle_permutation(s)
    )


def main():
    m = middle_permutation("abcdxg")
    print(m)


if __name__ == "__main__":
    main()

# source:
# https://codereview.stackexchange.com/questions/163292/finding-the-middle-permutation
# https://www.jianshu.com/p/f44ad7268efc
# https://medium.com/@aiswaryamathur/find-the-n-th-permutation-of-an-ordered-string-using-factorial-number-system-9c81e34ab0c8

# solutions:
# def middle_permutation(s):
#     s = ''.join(sorted(s, reverse=True))
#     return s[ len(s)//2 : (len(s)+3)//2 ] + s[ :len(s)//2 ] + s[ (len(s)+3)//2: ]
#
# def middle_permutation(string):
#     str, ln = sorted(string, reverse=True), len(string)
#     return ''.join([str.pop(ln//2), str.pop(ln//2)] + str) if ln % 2 else ''.join([str.pop(ln//2)] + str)
#
# def middle_permutation(s):
#     s = ''.join(sorted(s))
#     return s[len(s)//2-1:(len(s)+1)//2][::-1] + s[(len(s)+1)//2:][::-1] + s[:len(s)//2-1][::-1]

# def middle_permutation(string):
#     letters = sorted(list(string))
#     middle = letters.pop((len(letters)-1)//2)
#     if not len(letters) % 2:
#         return middle + middle_permutation("".join(i for i in letters))
#     return middle + "".join([i for i in letters[::-1]])


# def middle_permutation(string):
#     a = list(string)
#     a.sort()
#
#     # get middle char
#     result = [a[(len(a)-1) // 2]]
#     a.remove(a[(len(a)-1) // 2])
#
#     # if string length is even get m/2th char
#     if not (len(a) % 2):
#         result.append(a[len(a) // 2 - 1])
#         a.remove(a[len(a) // 2 - 1])
#     # pop off the end until empty
#     while a:
#         result.append(a.pop())
#     return ''.join(result)
