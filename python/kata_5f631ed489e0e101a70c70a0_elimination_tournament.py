# def check_rounds(d):
#     # if d == 1:
#     #     return 1
#     # return 1 + check_rounds(d // 2)
#     round = d
#     res = 0
#     while round != 1:
#         if round % 2 == 0:
#             res = res + 1
#             round = round // 2
#         else:
#             res = res + 1
#             round = (round // 2) + 1
#
#     return res


def play_round(input):
    # rounds = check_rounds(len(input))
    print(f"input len: {len(input)}")
    if len(input) == 1:
        return input

    split_size = 2
    splitted_list = []
    left_over = []
    if len(input) % 2 == 0:
        splitted_list = [
            input[i : i + split_size] for i in range(0, len(input), split_size)
        ]
    else:
        splitted_list = [
            input[i : i + split_size] for i in range(0, len(input), split_size)
        ]
        left_over = splitted_list.pop()[0]

    # print(f"splitted_list: {splitted_list}")
    winners = [max(i) for i in splitted_list]
    # print(left_over)
    # print(winners)
    if left_over:
        print(f"left_over: {left_over}")
        winners.insert(0, left_over)

    print(f"winners: {winners}")
    return winners
    # for i in range(0, rounds):
    #     pass


def tourney(input):
    # print(f"rounds: {check_rounds(len(input))}")
    input_list = input
    # rounds = (
    #     check_rounds(len(input))
    #     if len(input) % 2 == 0
    #     else (check_rounds(len(input)) + 1)
    # )
    # rounds = check_rounds(len(input))
    # print(f"calculated rounds: {rounds}")
    output = []
    # for x in range(1, rounds + 1):
    while 1:
        # print(f"start round: {x}")
        output.append(input_list)
        if len(input_list) == 1:
            break
        print(input_list)
        input_list = play_round(input_list)
        # print(f"end round: {x}\n")
    print(output)


def main():
    input = [9, 5, 4, 7, 6, 3, 8, 2]
    input2 = [
        75,
        42,
        7,
        12,
        33,
        71,
        96,
        92,
        97,
        16,
        41,
        14,
        68,
        12,
        88,
        29,
        15,
        39,
        48,
        90,
        15,
        55,
        45,
        82,
        55,
        100,
        22,
        50,
        48,
        88,
        1,
        63,
        38,
        70,
        90,
        84,
        37,
        98,
        8,
        92,
    ]
    # tourney(input2)
    tourney(input2)


if __name__ == "__main__":
    main()
