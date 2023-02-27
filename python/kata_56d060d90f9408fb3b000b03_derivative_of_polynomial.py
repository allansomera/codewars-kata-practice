der = []


def derivative(eq):
    global der
    terms = eq.split("+")
    # if len(eq.split("+")) == 1 and "x" not in eq:
    #     return str(0)
    for i in terms:
        x = check_term(i)
    # if len(der) == 1:
    #     der_string = "".join(der)
    # else:
    # der_string = f"{'+'.join(der)}"
    print(f"der is: {der}")
    filtered_list = [i for i in der if i != "0"]
    der_string = f"{'+'.join(filtered_list)}"
    der.clear()
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
        print(f"x^ found: {t2}")
        print(f"t2 is: {t2}")
        # t2 is now a list
        if "-" in t2[0]:
            print("negative sign detected")
            print(f"t2 is: {t2}")
            if len(t2[0]) == 1:
                # single x with negative sign '-x'
                print("single -x")
                print(f"t2 is: {t2}")
                coeff_i = -1
                exp_i = int(t2[1])
                exp_right = int(exp_i) - 1
                coeff_res = coeff_i * exp_i
                print(f"negative coeff only 1: {coeff_res}")
                if exp_right == 1:
                    term_res = f"{coeff_res}x"
                else:
                    term_res = f"{coeff_res}x^{exp_right}"
                der.append(term_res)
                print(f"{term_res}")
            else:
                print("negative number")
                print(f"t2 is: {t2}")
                coeff_i = int(t2[0])
                exp_i = int(t2[1])
                exp_right = int(exp_i) - 1
                coeff_res = coeff_i * exp_i
                print(f"negative coeff: {coeff_res}")
                if exp_right == 1:
                    term_res = f"{coeff_res}x"
                else:
                    term_res = f"{coeff_res}x^{exp_right}"
                # term_res = f"{coeff_res}x^{exp_right}"
                der.append(term_res)
                print(f"{term_res}")
        else:
            # no negative sign
            if t2[0] == "":
                t2[0] = 1
            print(f"t2 is: {t2}")
            print(f"t2[0] is: {t2[0]}")
            coeff_i = int(t2[0])
            exp_i = int(t2[1])
            exp_right = int(exp_i) - 1
            coeff_res = coeff_i * exp_i
            print(f"{coeff_res}x^{exp_right}")

            if exp_right == 1:
                term_res = f"{coeff_res}x"
            else:
                term_res = f"{coeff_res}x^{exp_right}"
            # term_res = f"{coeff_res}x^{exp_right}"
            der.append(term_res)
    elif "x" in t:
        # only 1 x
        t2 = t.split("x")
        print(f"only single x found: {t2}")
        print(f"t is: {t}")
        if "-" in t2[0]:
            if len(t2[0]) == 1:
                # single x with negative sign '-x'
                print(f"single x with negative sign")
                if t2[0] == "-" and t2[1] == "":
                    coeff_i = -1
                    exp_i = 1
                    # exp_right = int(exp_i) - 1
                    coeff_res = coeff_i * exp_i
                    der.append(f"{coeff_res}")
                    print(f"negative coeff only 1: {coeff_res}")
                else:
                    coeff_i = int(t2[0])
                    exp_i = 1
                    coeff_res = coeff_i * exp_i
                    der.append(f"{coeff_res}")
            else:
                coeff_i = int(t2[0])
                exp_i = int(t2[1])
                exp_right = int(exp_i) - 1
                coeff_res = coeff_i * exp_i
                der.append(f"{coeff_res}")
                print(f"negative coeff: {coeff_res}")
        else:
            # after split on single x, you find ['', '']
            print(f"single x, no negative sign")
            if t2[0] == "":
                coeff_i = 1
                term_res = f"{coeff_i}"
                der.append(term_res)
            else:
                coeff_i = int(t2[0])
                exp_i = 1
                coeff_res = coeff_i * exp_i
                # term_res = f"{coeff_res}"
                der.append(f"{coeff_res}")
                print(f"{coeff_res}\n")
    else:
        print("only coeff")
        # no x
        if "-" in t:
            print("no x, with negative sign")
            print(f"t is: {t}")
            der.append(str(coeff_res))
            pass
        else:
            print("no x")
            print(f"t is: {t}")
            if len(der) == 0:
                coeff_res = 0
                der.append(str(coeff_res))
            der.append(str(coeff_res))

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
    # print("5x^3+2x^2+2x+4")
    # print(derivative("-5x^3+2x^2+2x+4"))
    # print("-x^2+3x+4")
    # print(derivative("-x^2+3x+4"))
    # print("x^2+3x+4")
    # print(derivative("x^2+3x+4"))
    print("2x+100")
    print(derivative("2x+100"))
    # print("x+4")
    # print(derivative("x+4"))
    # print("-100")
    # print(derivative("-100"))
    # print(f"{'+'.join(der)}")

    # print("2x+1")
    # derivative("2x+1")


if __name__ == "__main__":
    main()
