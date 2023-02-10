function triangular(n) {
  let sum = 0
  if (n === 0) return 0
  for (let i = 1; i <= n; i++) {
    sum += i
  }
  return sum
}

triangular(4)
