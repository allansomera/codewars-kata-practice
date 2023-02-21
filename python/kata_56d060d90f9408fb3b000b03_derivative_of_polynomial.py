der = []


def derivative(eq):
    terms = eq.split("+")
    for i in terms:
        x = check_term(i)


def check_term(t):
    # check for exponent
    coeff = 0
    exp = 0
    exp_d = 0
    coeff_res = 0
    term_res = ""
    if "^" in t:
        # if true, asssume x is the character before '^'
        print(t.split("^"))
        exp_d = int(t.split("^")[1])
        print(f"exp_d: {exp_d}")
        print(t.split("^")[0].split("x"))
        coeff = int(t.split("^")[0].split("x")[0])
        print(f"coeff: {coeff}")
        if exp_d >= 1:
            exp = exp_d - 1
        else:
            exp = 0
        coeff_res = coeff * exp_d
        print(f"coeff_res: {coeff_res}")
        term_res = f"{coeff_res}x^{exp}"
        print(f"term_res: {term_res}\n")
    else:
        # no exponent '^'
        # check for x
        if "x" in t:
            print("has x in term")
            chk_x = t.split("x")
            print(chk_x)
            if len(chk_x) == 1:
                print("no x")
            # no x

    global der
    der.append(term_res)
    # print(der)

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

    print("2x+1")
    derivative("2x+1")


if __name__ == "__main__":
    main()
