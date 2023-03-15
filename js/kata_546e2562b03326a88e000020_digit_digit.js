function squareDigits(num) {
  return num
    .toString()
    .split("")
    .map((i) => parseInt(i) * parseInt(i))
    .join("")
}

n = squareDigits(3212)
console.log(n)
