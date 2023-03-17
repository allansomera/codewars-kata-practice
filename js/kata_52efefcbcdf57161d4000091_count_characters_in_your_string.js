const count = (string) => {
  // if (string === "") return {}
  // let o = {}
  // string.split("").forEach((i) => {
  //   return (o[i] = ++o[i] || 1)
  // })
  return string === ""
    ? ""
    : string.split("").reduce((ac, a) => ({ ...ac, [a]: ++ac[a] || 1 }), {})
}

console.log(count("a2"))

// nice solutions:
// const count = string =>
//   [...string].reduce((pre, val) => (pre[val] = -~pre[val], pre), {});
//
// function count (string) {
//   return string.split('').reduce(function(o,v){ o[v] ? o[v]++ : (o[v] = 1); return o; }, {});
// }
//
// function count (string) {
//   return string.split("").reduce(function (counts, char) {
//     counts[char] = ++counts[char] || 1;
//     return counts;
//   }, {});
// }
// more info:
// https://stackoverflow.com/questions/66863480/javascript-having-trouble-understanding-empty-initializer-in-reduce-functionkk
