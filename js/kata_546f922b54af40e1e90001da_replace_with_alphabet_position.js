function alphabetPosition(text) {
  let alpha = "abcdefghijklmnopqrstuvwxyz"
  return text
    .split("")
    .map((i) => alpha.indexOf(i.toLowerCase()) + 1)
    .filter((i) => i != 0)
    .join(" ")
}

// alphabetPosition("the sunset sets at twelve o' clock.")
console.log(alphabetPosition("The narwhal bacons at midnight."))

// solutions:
// function alphabetPosition(text) {
//   return text.toLowerCase().split('').map(a => a.charCodeAt(0) - 96).filter(a => a > 0 && a < 27).join(' ');
// }

// function alphabetPosition(text) {
//   return text
//     .toLowerCase()
//     .replace(/[^a-z]/g,'')
//     .replace(/./g, c => c + " ")
//     .replace(/[a-z]/g, c => c.charCodeAt(0) - 96)
//     .trim();
// }

// function alphabetPosition(text) {
//   return text.toLowerCase()
//     .split('')
//     .filter(c => c.charCodeAt(0) < 123 && c.charCodeAt(0) > 96)
//     .map(c => c.charCodeAt(0) - 96)
//     .join(' ');
// }
