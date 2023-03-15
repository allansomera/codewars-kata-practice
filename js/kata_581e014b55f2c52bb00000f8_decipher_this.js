function decipherThis(str) {
  let rgx = /^([0-9]+)(.*)/i
  let split_arr = str.split(" ")
  let m = split_arr.map((i) => {
    let a = i.match(rgx)[1]
    let b = i.match(rgx)[2]
    // console.log(i.match(rgx))
    // console.log(
    //   b.length >= 2
    //     ? String.fromCharCode(a) + b[b.length - 1] + b.slice(1, -1) + b[0]
    //     : String.fromCharCode(a) + b
    // )
    // console.log(
    //   String.fromCharCode(a) +
    //     (b.length >= 2 ? [b.length - 1] + b.slice(1, -1) + b[0] : b)
    // )
    return b.length >= 2
      ? String.fromCharCode(a) + b[b.length - 1] + b.slice(1, -1) + b[0]
      : String.fromCharCode(a) + b
  })
  // return m.toString().split(",").join(" ")
  return m.join(" ")
}

console.log(
  decipherThis("72eva 97 103o 97t 116sih 97dn 115ee 104wo 121uo 100o")
)

// solutions:
// function decipherThis(str) {
//   return str
//     .split(" ")
//     .map((w) =>
//       w
//         .replace(/^\d+/, (c) => String.fromCharCode(c))
//         .replace(/^(.)(.)(.*)(.)$/, "$1$4$3$2")
//     )
//     .join(" ")
// }
//
// const decipherThis = (str) =>
//   str.replace(
//     /\b(\d{2,3})(\w?)(\w*?)(\w?)\b/g,
//     (_, num, second, middle, last) =>
//       String.fromCharCode(num) + last + middle + second
//   )
//
// decipherThis = (s) =>
//   s
//     .replace(/\d+/g, (c) => String.fromCharCode(c))
//     .replace(/(\w)(\w)(\w*)(\w)/g, "$1$4$3$2")
