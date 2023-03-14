var countBits = function (n) {
  return n
    .toString(2)
    .split("")
    .filter((i) => i === "1").length
}

console.log(countBits(1234))

// solutions:
// var countBits = function(n) {
//   // Program Me
//   return n.toString(2).split("").reduce((a,b) => parseInt(a)+parseInt(b),0);
// };
//
// var countBits = function (n) {
//   var bits = 0;
//   for (; n; n >>= 1) {
//     if (n & 1) bits++;
//   }
//
//   return bits;
// };
//
//
// var countBits = function(n)
// {
//   a = n.toString(2).match(/1/g);
//   return a == null ? 0 : a.length;
// };
//
// var countBits = function(n) {
//   return n.toString(2).replace(/0/g,'').length;
// };
