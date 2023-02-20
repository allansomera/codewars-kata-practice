der = []


def derivative(eq):
    terms = eq.split("+")
    for i in terms:
        x = check_term(i)


def check_term(t):
    # check for exponent
    coeff = 0
    exp = 0
    coeff_res = 0
    if "^" in t:
        # if true, asssume x is the character before '^'
        print(t.split("^"))
        exp_s = t.split("^")[1]
        print(f"exp_d: {exp_s}")
        print(t.split("^")[0].split("x"))
        coeff = int(t.split("^")[0].split("x")[0])
        print(f"coeff: {coeff}\n")
        if int(exp_s) >= 1:
            exp -= int(exp_s)
        else:
            exp = 0

    # print(coeff)
    # print(exp)
    # else assume x^1
    # check if it has a coefficient
    # if true, multiply by 1, return product
    # else return 1
    # print(t.split("^"))
    pass


def main():
    # derivative("4x^2+2x+1")
    print("5x^3+2x^2+4")
    derivative("5x^3+2x^2+4")


if __name__ == "__main__":
    main()
