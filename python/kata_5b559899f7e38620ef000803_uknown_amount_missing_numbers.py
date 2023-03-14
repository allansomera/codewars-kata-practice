def miss_nums_finder(arr):
    arr2 = set(arr)
    res = [i for i in range(1,max(arr2)+1) if i not in arr2]
    return res
