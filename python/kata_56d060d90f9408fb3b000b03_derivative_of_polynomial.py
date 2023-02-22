der = []


def derivative(eq):
    global der
    terms = eq.split("+")
    for i in terms:
        x = check_term(i)
    der_string = f"{'+'.join(der)}"
    return der_string


def check_term(t):
    # check for exponent, split by 'x^'
    global der
    exp_i = 0
    coeff_i = 0
    coeff_res = 0
    exp_right = 0
    term_res = ""
    der_string = []
    if "^" in t:
        t2 = t.split("x^")
        # print(f"t2 is: {t2}")
        # t2 is now a list
        if "-" in t2[0]:
            print("negative sign detected")
            if len(t2[0]) == 1:
                # single x with negative sign '-x'
                print("single -x")
                coeff_i = -1
                exp_i = int(t2[1])
                exp_right = int(exp_i) - 1
                coeff_res = coeff_i * exp_i
                print(f"negative coeff only 1: {coeff_res}")
                term_res = f"{coeff_res}x^{exp_right}"
                der.append(term_res)
                print(f"{term_res}")
            else:
                print("negative number")
                coeff_i = int(t2[0])
                exp_i = int(t2[1])
                exp_right = int(exp_i) - 1
                coeff_res = coeff_i * exp_i
                print(f"negative coeff: {coeff_res}")
                term_res = f"{coeff_res}x^{exp_right}"
                der.append(term_res)
                print(f"{term_res}")
        else:
            # print(f"t2 is: {t2}")
            coeff_i = int(t2[0])
            exp_i = int(t2[1])
            exp_right = int(exp_i) - 1
            coeff_res = coeff_i * exp_i
            print(f"{coeff_res}x^{exp_right}")
            term_res = f"{coeff_res}x^{exp_right}"
            der.append(term_res)
    elif "x" in t:
        # only 1 x
        t2 = t.split("x")
        if "-" in t2[0]:
            if len(t2[0]) == 1:
                # single x with negative sign '-x'
                coeff_i = -1
                exp_i = int(t2[1])
                exp_right = int(exp_i) - 1
                coeff_res = coeff_i * exp_i
                print(f"negative coeff only 1: {coeff_res}")
            else:
                coeff_i = int(t2[0])
                exp_i = int(t2[1])
                exp_right = int(exp_i) - 1
                coeff_res = coeff_i * exp_i
                print(f"negative coeff: {coeff_res}")
        else:
            coeff_i = int(t2[0])
            exp_i = 1
            coeff_res = coeff_i * exp_i
            term_res = f"{coeff_res}"
            der.append(term_res)
            print(f"{coeff_res}\n")
    else:
        coeff_res = 0

    # global der
    # der_string = "+".join(der)
    # return der_string

    # print(coeff)
    # print(exp)
    # else assume x^1
    # check if it has a coefficient
    # if true, multiply by 1, return product
    # else return 1
    # print(t.split("^"))


def main():
    # derivative("4x^2+2x+1")
    print("5x^3+2x^2+2x+4")
    print(derivative("-5x^3+2x^2+2x+4"))
    # print("-x^3+2x^2+2x+4")
    # print(derivative("-x^3+2x^2+2x+4"))
    # print(f"{'+'.join(der)}")

    # print("2x+1")
    # derivative("2x+1")


if __name__ == "__main__":
    main()
