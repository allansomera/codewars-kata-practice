var uniqueInOrder = function (iterable) {
  if (Array.isArray(iterable)) {
    return iterable.every((i) => typeof i === "number")
      ? [
          ...iterable
            .toString()
            .split(",")
            .join("")
            .replace(/(.)\1+/g, "$1"),
        ].map(Number)
      : [
          ...iterable
            .toString()
            .split(",")
            .join("")
            .replace(/(.)\1+/g, "$1"),
        ]
  } else
    return [
      ...iterable.toString().replace(/(.)\1+/g, "$1"),
      // .split(""),
    ]
}

// console.log(uniqueInOrder("AAAABBBCCDAABBB"))
// console.log(uniqueInOrder([1, 2, 2, 3, 3]))
console.log(uniqueInOrder(["a", "a", "b", "c"]))

// good solution using reduce
//
// const reducer = (acc, x) =>
//   acc[acc.length - 1] === x
//     ? acc
//     : [...acc, x]
//
// const uniqueInOrder = x => [].reduce.call(x, reducer, [])
